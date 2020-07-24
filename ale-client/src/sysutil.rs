use crate::config::Proc;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use sysinfo::{ProcessExt, ProcessorExt, System, SystemExt};

type Result<T> = std::result::Result<T, ProcessNotFoundError>;

#[derive(Debug)]
/// An Error for process not found
pub struct ProcessNotFoundError {
    proc_name: String,
    pid: i32,
}

impl fmt::Display for ProcessNotFoundError {
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
    pub proc_map: Vec<ProcInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcInfo {
    pub name: String,
    pub pid: i32,
    pub used_memory: u64,
    pub used_virtual: u64,
    pub cpu_usage: f32,
    pub read_bytes: u64,
    pub total_read_bytes: u64,
    pub written_bytes: u64,
    pub total_written_bytes: u64,

    #[serde(skip_serializing)]
    pub pid_filepath: String,

    #[serde(skip_serializing)]
    pub retry_count: i32,

    #[serde(skip_serializing)]
    pub disabled: bool,
}

impl ProcInfo {
    pub fn update(&mut self, sys: &System, pid: i32) -> Result<()> {
        log::debug!("PROCESS INFO :: {:#?}", sys.get_process(pid));
        sys.get_process(pid).map_or(
            {
                self.reset();
                Err(ProcessNotFoundError {
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

    pub fn new(name: String, pid: i32, path: String) -> Self {
        ProcInfo {
            pid,
            name,
            pid_filepath: path,
            used_memory: 0,
            used_virtual: 0,
            cpu_usage: 0.0,
            read_bytes: 0,
            total_read_bytes: 0,
            written_bytes: 0,
            total_written_bytes: 0,
            retry_count: 0,
            disabled: false,
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
    pub info: SystemInfo,
    pub max_retry: i32,
}

impl SystemUtil {
    pub fn with(sys: System, procs_info: Vec<Proc>, max_retry: i32) -> Self {
        SystemUtil {
            sys,
            info: SystemInfo {
                last_updated: Local::now().timestamp(),
                proc_map: procs_info
                    .iter()
                    .map(|p| {
                        ProcInfo::new(
                            p.name.to_string(),
                            p.pid,
                            p.path
                                .as_ref()
                                .map_or("".to_string(), |f| f.to_str().unwrap().to_string()),
                        )
                    })
                    .collect(),
                total_memory: 0,
                used_memory: 0,
                total_swap: 0,
                used_swap: 0,
                cpu_usage: 0.0,
            },
            max_retry,
        }
    }

    pub async fn get_sys_info(&mut self) -> &SystemInfo {
        self.sys.refresh_all();
        self.info.last_updated = Local::now().timestamp();
        self.info.total_memory = self.sys.get_total_memory() as i64;
        self.info.used_memory = self.sys.get_used_memory() as i64;
        self.info.total_swap = self.sys.get_total_swap() as i64;
        self.info.used_swap = self.sys.get_used_swap() as i64;
        self.info.cpu_usage = self.sys.get_global_processor_info().get_cpu_usage();

        for (_i, p) in self
            .info
            .proc_map
            .iter_mut()
            .filter(|p| !p.disabled)
            .enumerate()
        {
            match p.update(&self.sys, p.pid) {
                Ok(_) => (),
                Err(e) => {
                    log::error!("encountered error: {:?}, retrying...", e);
                    if !p.disabled {
                        p.retry_count += 1;
                        if !p.pid_filepath.is_empty() {
                            let pid = fs::read_to_string(p.pid_filepath.as_str());
                            if let Ok(pid) = pid {
                                log::debug!("got from file:: {}", pid);
                                let pid: i32 = pid.parse().unwrap(); //TODO use ? to handle this properly
                                if pid != p.pid {
                                    p.retry_count = 0;
                                    log::info!("new pid for {} found :: {}", p.name, pid);
                                    p.pid = pid;
                                }
                            }
                        }

                        if p.retry_count > self.max_retry || p.pid_filepath.is_empty() {
                            p.disabled = true;
                        }
                    }
                }
            }
        }

        &self.info
    }
}
