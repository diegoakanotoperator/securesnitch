use sysinfo::{System, SystemExt, ProcessExt};
use crate::hashing;

pub struct ProcessInfo {
    pub pid: u32,
    pub ppid: u32,
    pub uid: u32,
    pub comm: String,
    pub path: String,
    pub args: Vec<String>,
    pub hash: Option<String>,
}

pub fn get_process_info(pid: u32) -> Option<ProcessInfo> {
    let mut sys = System::new_all();
    sys.refresh_processes();

    if let Some(process) = sys.process(pid.into()) {
        let path = process.exe().to_string_lossy().into_owned();
        let hash = hashing::calculate_sha256(&path).ok();

        return Some(ProcessInfo {
            pid,
            ppid: process.parent().map(|p| p as u32).unwrap_or(0),
            uid: 0, // Sysinfo handles UID differently per platform
            comm: process.name().to_string(),
            path,
            args: process.cmd().to_vec(),
            hash,
        });
    }
    None
}