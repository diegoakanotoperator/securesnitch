#[cfg(target_os = "linux")]
use aya::Ebpf;
use std::path::Path;

#[cfg(target_os = "linux")]
pub fn load_ebpf_module<P: AsRef<Path>>(path: P) -> Result<Ebpf, aya::EbpfError> {
    // aya provides Ebpf::load_file which natively parses the ELF
    Ebpf::load_file(path)
}

#[cfg(not(target_os = "linux"))]
pub fn load_ebpf_module<P: AsRef<Path>>(_path: P) -> anyhow::Result<()> {
    Err(anyhow::anyhow!("eBPF is only supported on Linux"))
}

#[cfg(test)]
#[path = "ebpf_test.rs"]
mod tests;