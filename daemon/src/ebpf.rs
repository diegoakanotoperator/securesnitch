use std::path::Path;
use aya::Ebpf;

pub fn load_ebpf_module<P: AsRef<Path>>(path: P) -> Result<Ebpf, aya::EbpfError> {
    // aya provides Ebpf::load_file which natively parses the ELF
    Ebpf::load_file(path)
}

#[cfg(test)]
#[path = "ebpf_test.rs"]
mod tests;