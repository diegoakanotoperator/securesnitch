# SecureSnitch

SecureSnitch is a high-performance, memory-safe, and hardened host-based application firewall for Linux, originally forked from [evilsocket/opensnitch](https://github.com/evilsocket/opensnitch).

The project has been completely refactored from Go and Python into a unified Rust architecture, integrating **Litebox** memory enclaves and **SQLCipher** for data-at-rest encryption.

## Features
- **Fail-Closed Security:** Prevents network leaks before the firewall rules are loaded.
- **eBPF-Powered Monitoring:** High-performance kernel-level tracking of system events.
- **Advanced Rules Engine:** Support for complex nested conditions (Regex, CIDR, numeric ranges).
- **Anti-Tampering:** Memory protection for the rules engine using the Litebox enclave.
- **Modern Iced GUI:** A fast, native Rust dashboard and system tray icon.

## Prerequisites
- Rust (latest stable)
- Linux Kernel with eBPF and NFQUEUE support.
- `pkg-config`, `libdbus-1-dev`, `libgtk-3-dev` (for UI and system tray).

## Building
```bash
cargo build --release
```

## Running
1. Start the daemon as root:
   ```bash
   sudo ./target/release/daemon
   ```
2. Start the UI:
   ```bash
   ./target/release/ui
   ```

## Security Posture
SecureSnitch remediates several vulnerabilities found in the legacy codebase (notably SQL Injection and OS Command Injection) by utilizing Rust's memory safety and strictly parameterized queries. See `docs/testing_and_pentest_report.md` for more details.
