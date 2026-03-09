# 🏗️ SecureSnitch Architecture

## System Overview
SecureSnitch employs a high-integrity, multi-process architecture consisting of a privileged system daemon and a user-space graphical interface.

## ⚙️ Core Components

### 1. The SecureSnitch Daemon (`daemon/`)
The "Brain" of the firewall. It runs with elevated privileges to interact with the kernel.
- **Event Processor:** Uses **eBPF** (`aya`) to track process execution and lifecycle events.
- **Network Interceptor:** Uses **NFQUEUE** (`nfq`) to pause outbound packets until a policy decision is made.
- **Security Enclave:** Uses **Litebox** patterns to isolate the rules engine in a memory-guarded space.
- **Privilege Manager:** Drops all unnecessary root capabilities after initialization, retaining only `CAP_NET_ADMIN` and `CAP_BPF`.

### 2. The SecureSnitch UI (`ui/`)
A native, cross-platform dashboard built with the **Iced** framework.
- **gRPC Client:** Communicates with the daemon via an encrypted, **mTLS-authenticated** gRPC channel.
- **Rule Manager:** Interfaces with the **SQLCipher** encrypted SQLite database.
- **Sysbar Gadget:** A background-resident system tray icon for real-time notifications.

### 3. DNS Protector
A built-in proxy that intercepts local DNS requests and upgrades them to **DNS-over-HTTPS (DoH)**, ensuring that filtering remains effective even against applications attempting to bypass local DNS.

## 🔒 Security Hardening Data Flow
1.  **Binary Execution:** eBPF captures the `execve` event and computes the **SHA-256 hash** of the binary on disk.
2.  **Network Request:** NFQ holds the packet.
3.  **Contextual Lookup:** The daemon fetches the process lineage (PPID chain) and maps it to the intercepted connection.
4.  **Enclave Validation:** The rules engine (inside the Litebox enclave) matches the Connection + Hash + Parent Process against the policy.
5.  **Final Verdict:** Packet is released or dropped based on the result.

## 🛠️ Cross-Platform Strategy
SecureSnitch uses a trait-based backend abstraction (`backend.rs`).
- **Linux:** Uses native Netfilter and eBPF.
- **macOS/Windows:** Uses a generic `sysinfo` process tracker with experimental platform-specific interception hooks.
