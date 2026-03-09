# 🛡️ SecureSnitch

**The Fortified, Memory-Safe Application Firewall for the Modern Desktop.**

SecureSnitch is a next-generation, host-based application firewall for Linux, macOS, and Windows. Originally forked from the pioneering [OpenSnitch](https://github.com/evilsocket/opensnitch) project, SecureSnitch has been completely rebuilt from the ground up in **Rust** to provide uncompromising performance, memory safety, and advanced anti-tampering protections.

---

## 🚀 Mission
Our mission is to provide users with absolute transparency and control over their system's network activity while ensuring the firewall itself is as resilient as the kernel it protects.

## 💎 Key Pillars

### 🛡️ Hardened Security
- **Litebox Enclaves:** Core policy logic is isolated in a secure memory enclave to prevent in-memory tampering and process injection.
- **Binary Integrity:** Every rule is tied to a unique SHA-256 binary hash. If a trusted application is modified or replaced, SecureSnitch detects it instantly.
- **Fail-Closed Boot:** Our "Gatekeeper" strategy ensures zero network leaks during the boot process by redirecting traffic to NFQUEUE before the network stack is fully active.

### 🦀 Rust Powered
- **Memory Safety:** Elimination of buffer overflows, use-after-free, and other memory-class vulnerabilities common in C/C++ implementations.
- **eBPF Integration:** High-performance, pure-Rust kernel monitoring via the `aya` crate.
- **Native Speed:** Minimal CPU and memory overhead, even under heavy network load.

### 🌐 Privacy First
- **Encrypted Storage:** Rules and logs are secured at rest using **SQLCipher** (AES-256).
- **Encrypted DNS:** A built-in **DNS-over-HTTPS (DoH)** proxy prevents DNS hijacking and protects your browsing metadata from local ISP snooping.

---

## 📦 Supported Platforms
- **Linux:** Full eBPF + NFQUEUE support (x86_64).
- **macOS:** Intel & Apple Silicon support (Build stable, Backend experimental).
- **Windows:** x86_64 support (Build stable, Backend experimental).

## 🛠️ Quick Start

### Prerequisites
- Rust (latest stable)
- Linux: `libdbus-1-dev`, `libgtk-3-dev`, `pkg-config`, `libxdo-dev` (optional)

### Build
```bash
cargo build --release
```

### Run
```bash
# Start the hardened daemon
sudo ./target/release/daemon

# Start the Iced-based UI
./target/release/ui
```

---

## 🤝 Community & Collaboration
SecureSnitch is an open-source project. We welcome contributions that align with our pillars of security, performance, and privacy. Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📜 Acknowledgements
SecureSnitch is a modernized fork of [evilsocket/opensnitch](https://github.com/evilsocket/opensnitch). We are grateful to the original authors for their vision and community groundwork.
