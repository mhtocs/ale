use std::fmt;
use sysinfo::{ProcessExt, ProcessorExt, System, SystemExt};

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

pub struct SystemInfo {
    total_memory: u64,
    used_memory: u64,
    total_swap: u64,
    used_swap: u64,
    cpu_usage: f32,
    ela: ProcInfo,
    es: ProcInfo,
    sysevt: ProcInfo,
}

pub struct ProcInfo {
    name: String,
    used_memory: u64,
    used_virtual: u64,
    cpu_usage: f32,
    read_bytes: u64,
    total_read_bytes: u64,
    written_bytes: u64,
    total_written_bytes: u64,
}

type Result<T> = std::result::Result<T, ProcNotFound>;

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
                self.cpu_usage = proc.cpu_usage();

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
        self.total_read_bytes = 0;
        self.written_bytes = 0;
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
    pub fn from(sys: System, ela_pid: i32, es_pid: i32, sysevt_pid: i32) -> Self {
        SystemUtil {
            sys,
            ela_pid,
            es_pid,
            sysevt_pid,
            info: SystemInfo {
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

    pub fn get_sys_info(mut self) -> SystemInfo {
        self.sys.refresh_all();
        self.info.total_memory = self.sys.get_total_memory();
        self.info.used_memory = self.sys.get_used_memory();
        self.info.total_swap = self.sys.get_total_swap();
        self.info.used_swap = self.sys.get_used_swap();
        self.info.cpu_usage = self.sys.get_global_processor_info().get_cpu_usage();
        self.info.ela.update(&self.sys, self.ela_pid).unwrap();
        self.info.es.update(&self.sys, self.es_pid).unwrap();
        self.info.sysevt.update(&self.sys, self.sysevt_pid).unwrap();
        self.info
    }
}
