# SecureSnitch Rust & Litebox Refactoring Plan

## Overview
This document outlines the strategy for refactoring the SecureSnitch host-based application firewall. The goal is to transition the Go daemon and Python/C++ UI into a unified Rust architecture, utilizing the `iced` framework for the frontend and integrating `litebox` enclaves to provide robust security and anti-tampering measures.

## Phased Approach

### Phase 1: Project Setup & eBPF Integration
*   [x] **Task 1.1:** Initialize a new Rust workspace with `daemon` and `ui` crates.
*   [x] **Task 1.2:** Port the eBPF loading and attachment logic from Go to Rust (using `aya` or `libbpf-rs`).
*   [x] **Task 1.3:** Connect the existing C eBPF programs (`ebpf_prog/`) to the new Rust daemon to capture system events.

### Phase 2: Core Daemon Logic & Firewall
*   [x] **Task 2.1:** Implement Netfilter Queue (NFQUEUE) integration in Rust.
*   [x] **Task 2.2:** Port system tracking (procfs reading, Netlink sockets) to identify processes initiating connections.
*   [x] **Task 2.3:** Implement the rules engine and firewall backend (iptables/nftables) securely in Rust.

### Phase 3: Security & Litebox Enclave Integration
*   [x] **Task 3.1:** Integrate Litebox to isolate critical daemon components (like the rules engine and policy decision logic) within a secure enclave to guard against memory tampering.
*   [x] **Task 3.2:** Implement memory protection and anti-tampering guards for the daemon process.
*   [x] **Task 3.3:** Establish a secure, encrypted gRPC/IPC channel over Unix sockets between the enclave-protected daemon and the UI.

### Phase 4: Iced Frontend Development
*   [x] **Task 4.1:** Scaffold the Rust `iced` GUI application.
*   [x] **Task 4.2:** Implement the IPC client to communicate with the Rust daemon.
*   [x] **Task 4.3:** Build the main dashboard, events list, and rule management views.
*   [x] **Task 4.5:** Create a desktop system tray gadget (sysbar) for background monitoring and quick access.
*   [x] **Task 5.1:** Migrate the existing SQLite database to a safe Rust SQL library (e.g., `sqlx`) with SQLCipher encryption, directly addressing the high-severity SQL injection and protecting data at rest.
*   [x] **Task 5.2:** Ensure feature parity for background tasks (e.g., DNS tracking).
*   [x] **Task 5.3:** Comprehensive testing, packaging, and final integration.

### Phase 6: Advanced Security & Hardening
*   [x] **Task 6.1:** Implement Binary Integrity Verification (SHA-256 Checksumming).
*   [x] **Task 6.2:** Implement Process Tree Lineage (PPID Chain Tracking).
*   [x] **Task 6.3:** Implement DNS-over-HTTPS (DoH) / DNS-over-TLS (DoT) Proxy.
*   [x] **Task 6.4:** Implement Kernel-Level Self-Defense (eBPF LSM Hooks).
*   [x] **Task 6.5:** Implement Privilege Dropping (Linux Capabilities).