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
}