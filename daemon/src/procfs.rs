use sysinfo::{System, Pid};
use crate::hashing;

pub struct ProcessInfo {
    pub pid: u32,
    pub ppid: u32,
    pub uid: u32,
    pub comm: String,
    pub path: String,
    pub args: Vec<String>,
    pub hash: Option<String>,
    pub parent_path: Option<String>,
}

pub fn get_process_info(pid: u32) -> Option<ProcessInfo> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let pid_sys = Pid::from(pid as usize);
    if let Some(process) = sys.process(pid_sys) {
        let path = process.exe().map(|p| p.to_string_lossy().into_owned()).unwrap_or_default();
        let hash = hashing::calculate_sha256(&path).ok();
        let parent_path = process.parent()
            .and_then(|ppid| sys.process(ppid))
            .and_then(|p| p.exe())
            .map(|p| p.to_string_lossy().into_owned());

        return Some(ProcessInfo {
            pid,
            ppid: process.parent().map(|p| p.as_u32()).unwrap_or(0),
            uid: 0,
            comm: process.name().to_string_lossy().into_owned(),
            path,
            args: process.cmd().iter().map(|s| s.to_string_lossy().into_owned()).collect(),
            hash,
            parent_path,
        });
    }
    None
}