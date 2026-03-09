# SecureSnitch Rust Architecture

## Overview
The SecureSnitch Rust implementation follows a multi-process architecture designed for high performance, memory safety, and anti-tampering security.

## Components

### 1. eBPF Kernel Modules (`ebpf_prog/`)
*   **Role:** High-performance event capture within the Linux kernel.
*   **Hooks:** `execve`, `execveat`, and socket lifecycle events.
*   **Rust Integration:** The `daemon` crate uses the `aya` library to load and interact with these programs without requiring `libelf` or other system C dependencies.

### 2. SecureSnitch Daemon (`daemon/`)
*   **Role:** The core engine that processes kernel events and enforces firewall policies.
*   **Netfilter Queue (NFQ):** Intercepts network packets and holds them until a verdict (Allow/Deny) is reached.
*   **Rules Engine:** Evaluates intercepted events against the user-defined policy stored in the encrypted SQLite database.
*   **Litebox Enclave:** Protects the memory space of the rules engine and policy decision logic from external tampering and process injection.
*   **Procfs Monitoring:** Tracks the process tree and identifies the specific PID/Inode associated with network connections.

### 3. Iced UI (`ui/`)
*   **Role:** The graphical user interface for real-time monitoring and rule management.
*   **Framework:** Built using the `iced` native Rust GUI library.
*   **IPC Client:** Communicates with the daemon via an encrypted gRPC channel over Unix Domain Sockets.
*   **System Tray (Sysbar):** Provides a background resident icon for quick status checks and dashboard access.

### 4. Security Layer
*   **Database Encryption:** Uses SQLCipher to protect rules and logs at rest.
*   **Parameterized Queries:** Eliminates SQL Injection risks identified in the legacy Python implementation.
*   **Memory Safety:** The entire user-space stack is implemented in Rust, removing common C/C++ memory vulnerabilities.

## Data Flow
1.  **Event Capture:** eBPF detects a process making a connection.
2.  **Interception:** NFQUEUE holds the packet.
3.  **Process Identification:** Daemon maps the network event to a process name and path via `/proc`.
4.  **Policy Lookup:** Daemon checks the encrypted rules database.
5.  **User Interaction (Optional):** If no rule exists, an IPC request is sent to the UI to display a prompt.
6.  **Verdict:** The daemon sends an `ACCEPT` or `DROP` verdict to NFQUEUE.
