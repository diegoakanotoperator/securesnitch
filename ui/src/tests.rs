#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_title() {
        let ui = SecureSnitchUI::default();
        assert_eq!(ui.title(), "SecureSnitch - Rust & Iced");
    }

    // Smoke Test
    #[test]
    fn smoke_test_ui_init() {
        let _ui = SecureSnitchUI::default();
        // Just verifying initialization works without panic
        assert!(true);
    }
}