#[cfg(test)]
mod tests {
    use crate::litebox::Enclave;

    #[test]
    fn test_enclave_initialization() {
        let enclave = Enclave::new();
        assert!(enclave.initialized, "Enclave should be initialized");
    }
}