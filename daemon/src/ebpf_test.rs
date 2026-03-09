#[cfg(test)]
mod tests {
    use crate::ebpf::load_ebpf_module;
    use std::path::Path;

    // Unit test: Check if a non-existent eBPF module returns an error gracefully
    #[test]
    fn test_load_ebpf_module_not_found() {
        let path = Path::new("/path/that/does/not/exist.o");
        let result = load_ebpf_module(path);
        assert!(result.is_err(), "Loading a non-existent module should fail");
    }
}