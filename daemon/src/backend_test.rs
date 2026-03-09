#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os = "macos")]
    fn test_macos_backend_initialization() {
        let backend = get_backend().expect("Should return a backend on macOS");
        assert!(backend.initialize().is_ok(), "macOS stub backend should initialize");
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_windows_backend_initialization() {
        let backend = get_backend().expect("Should return a backend on Windows");
        assert!(backend.initialize().is_ok(), "Windows stub backend should initialize");
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_linux_backend_initialization() {
        // This might fail if not root, but we test the logic
        let backend = get_backend().expect("Should return a backend on Linux");
        // We don't assert init here as it requires root for NFQ/eBPF
    }
}