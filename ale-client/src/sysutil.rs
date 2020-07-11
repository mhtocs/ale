use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fmt;
use sysinfo::{ProcessExt, ProcessorExt, System, SystemExt};

type Result<T> = std::result::Result<T, ProcNotFound>;

#[derive(Debug)]
/// An Error for process not found
pub struct ProcNotFound {
    proc_name: String,
    pid: i32,
}

impl fmt::Display for ProcNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Process :: {} with pid : {} not found!",
            self.proc_name, self.pid
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub last_updated: i64,
    pub total_memory: i64,
    pub used_memory: i64,
    pub total_swap: i64,
    pub used_swap: i64,
    pub cpu_usage: f32,
    pub ela: ProcInfo,
    pub es: ProcInfo,
    pub sysevt: ProcInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcInfo {
    pub name: String,
    pub used_memory: u64,
    pub used_virtual: u64,
    pub cpu_usage: f32,
    pub read_bytes: u64,
    pub total_read_bytes: u64,
    pub written_bytes: u64,
    pub total_written_bytes: u64,
}

impl ProcInfo {
    pub fn update(&mut self, sys: &System, pid: i32) -> Result<()> {
        sys.get_process(pid).map_or(
            {
                self.reset();
                Err(ProcNotFound {
                    pid,
                    proc_name: self.name.to_string(),
                })
            },
            |proc| {
                self.used_memory = proc.memory();
                self.used_virtual = proc.virtual_memory();
                self.cpu_usage = proc.cpu_usage() / sys.get_processors().len() as f32;

                let du = proc.disk_usage();

                self.total_read_bytes = du.total_read_bytes;
                self.read_bytes = du.read_bytes;
                self.total_written_bytes = du.total_written_bytes;
                self.written_bytes = du.written_bytes;

                Ok(())
            },
        )
    }

    pub fn new(name: &str) -> Self {
        ProcInfo {
            name: name.to_string(),
            used_memory: 0,
            used_virtual: 0,
            cpu_usage: 0.0,
            read_bytes: 0,
            total_read_bytes: 0,
            written_bytes: 0,
            total_written_bytes: 0,
        }
    }

    pub fn reset(&mut self) {
        self.used_virtual = 0;
        self.used_memory = 0;
        self.cpu_usage = 0.0;
        self.read_bytes = 0;
        self.written_bytes = 0;
        self.total_read_bytes = 0;
        self.total_written_bytes = 0;
    }
}

pub struct SystemUtil {
    pub sys: System,
    pub ela_pid: i32,
    pub es_pid: i32,
    pub sysevt_pid: i32,
    pub info: SystemInfo,
}

impl SystemUtil {
    pub fn with(sys: System, ela_pid: i32, es_pid: i32, sysevt_pid: i32) -> Self {
        SystemUtil {
            sys,
            ela_pid,
            es_pid,
            sysevt_pid,
            info: SystemInfo {
                last_updated: Local::now().timestamp(),
                ela: ProcInfo::new("ela"),
                es: ProcInfo::new("es"),
                sysevt: ProcInfo::new("sysevt"),
                total_memory: 0,
                used_memory: 0,
                total_swap: 0,
                used_swap: 0,
                cpu_usage: 0.0,
            },
        }
    }

    pub fn get_sys_info(&mut self) -> &SystemInfo {
        self.sys.refresh_all();
        self.info.last_updated = Local::now().timestamp();
        self.info.total_memory = self.sys.get_total_memory() as i64;
        self.info.used_memory = self.sys.get_used_memory() as i64;
        self.info.total_swap = self.sys.get_total_swap() as i64;
        self.info.used_swap = self.sys.get_used_swap() as i64;
        self.info.cpu_usage = self.sys.get_global_processor_info().get_cpu_usage();
        self.info.ela.update(&self.sys, self.ela_pid).unwrap();
        self.info.es.update(&self.sys, self.es_pid).unwrap();
        self.info.sysevt.update(&self.sys, self.sysevt_pid).unwrap();
        &self.info
    }
}
