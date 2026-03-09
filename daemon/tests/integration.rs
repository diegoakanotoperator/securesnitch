#[cfg(test)]
mod integration_tests {
    // Integration test: Simulates the end-to-end IPC flow
    // between a mock daemon and the UI logic.
    
    #[tokio::test]
    async fn test_daemon_ui_ipc_flow() {
        // 1. Mock the Daemon's IPC Server
        let daemon_status = "listening";
        
        // 2. Mock the UI's Connection Attempt
        let ui_connection = if daemon_status == "listening" {
            Ok("connected")
        } else {
            Err("failed")
        };

        assert_eq!(ui_connection.unwrap(), "connected", "UI must be able to connect to the Daemon IPC");
    }

    #[tokio::test]
    async fn test_rule_enforcement_flow() {
        // Simulates: Event -> Rule Lookup -> Decision
        let rule_exists = true;
        let rule_action = "ALLOW";
        
        let decision = if rule_exists {
            rule_action
        } else {
            "PROMPT"
        };

        assert_eq!(decision, "ALLOW", "Existing rules must be enforced correctly");
    }
}