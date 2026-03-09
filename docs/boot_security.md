# Boot-Time Security & "Fail-Closed" Strategy

To ensure SecureSnitch is effective from the first second of system boot, the following "Fail-Closed" strategy is implemented.

## 1. Systemd Integration
The SecureSnitch daemon is registered as a systemd service with `Before=network.target`. This ensures the rules engine is initialized before the network stack is fully up.

## 2. Preventing Network Leaks (Fail-Closed)
To prevent applications from sending data before the daemon has finished loading its ruleset, we use a "Gatekeeper" approach:

1.  **Early Ruleset:** A minimal iptables/nftables ruleset is loaded via `ExecStartPre` in the systemd unit.
2.  **NFQUEUE Redirection:** This ruleset sends all `OUTPUT` traffic to `NFQUEUE` 0.
3.  **Daemon Takeover:** Once the daemon starts, it binds to `NFQUEUE` 0 and begins processing the backlog of packets based on the user's permanent rules.

## 3. Auto-Registration Logic
On installation, the following steps are performed:
1.  The daemon binary is placed in `/usr/local/bin/`.
2.  The `securesnitch.service` unit is copied to `/etc/systemd/system/`.
3.  The service is enabled: `systemctl enable securesnitch`.

## 4. Security Verification
During a boot-time audit:
- **Test:** Verify that a `curl` command executed in a separate early-init service is held until the SecureSnitch daemon is ready.
- **Expected Result:** The packet is buffered in the kernel NFQUEUE and only released after the daemon applies the "Allow" or "Deny" verdict.
