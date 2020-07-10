use sysinfo::{Process, System, SystemExt};

struct SystemInfo<'a> {
    total_memory: u64,
    used_memory: u64,
    total_swap: u64,
    used_swap: u64,
    cpu_usage: f32,
    ela: Option<&'a Process>,
    es: Option<&'a Process>,
    sysevt: Option<&'a Process>,
}

pub struct SystemUtil<'a> {
    sys: System,
    info: SystemInfo<'a>,
    ela_pid: i32,
    es_pid: i32,
    sysevt_pid: i32,
}

impl<'a> SystemUtil<'a> {
    pub fn from(sys: System, ela_pid: i32, es_pid: i32, sysevt_pid: i32) -> Self {
        SystemUtil {
            sys,
            ela_pid,
            es_pid,
            sysevt_pid,
            info: SystemInfo {
                total_memory: 0,
                used_memory: 0,
                total_swap: 0,
                used_swap: 0,
                cpu_usage: 0.0,
                ela: None,
                es: None,
                sysevt: None,
            },
        }
    }

    pub async fn get_sys_info(mut self) {
        self.sys.refresh_all();
        self.info.total_memory = self.sys.get_total_memory();
        self.info.used_memory = self.sys.get_used_memory();
        self.info.total_swap = self.sys.get_total_swap();
        self.info.used_swap = self.sys.get_used_swap();
        self.info.ela = self.sys.get_process(self.ela_pid);
        self.info.es = self.sys.get_process(self.es_pid);
        self.info.sysevt = self.sys.get_process(self.sysevt_pid);
    }
}
