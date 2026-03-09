#[cfg(test)]
mod tests {
    use crate::hashing::calculate_sha256;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_calculate_sha256() {
        let mut tmp_file = NamedTempFile::new().unwrap();
        write!(tmp_file, "SecureSnitch").unwrap();
        
        let path = tmp_file.path();
        let hash = calculate_sha256(path).expect("Failed to calculate hash");
        
        assert_eq!(hash.len(), 64);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_calculate_sha256_windows_binary() {
        // Simple test to ensure logic works on windows-style paths
        let mut tmp_file = NamedTempFile::new().unwrap();
        write!(tmp_file, "WindowsExecutable").unwrap();
        let path = tmp_file.path();
        let hash = calculate_sha256(path).unwrap();
        assert_eq!(hash.len(), 64);
    }
}