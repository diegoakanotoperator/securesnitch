# 🚩 SecureSnitch Red Team Audit (Attack Perspective)

## 1. Objective
Identify bypass techniques and persistence methods an attacker might use against SecureSnitch.

## 2. Attack Vectors

### 2.1 Process Impersonation
- **Attack:** An attacker replaces a trusted binary (e.g., `/usr/bin/curl`) with a malicious tool.
- **Defense:** SecureSnitch matches the binary's SHA-256 hash. If changed, the rule is invalidated and the user is prompted.
- **Verdict:** Highly Resilient.

### 2.2 Daemon Termination
- **Attack:** Attempting to `kill -9` the `securesnitch-daemon`.
- **Defense:** Kernel-level self-defense via eBPF LSM hooks prevents signals from being delivered to the daemon process.
- **Verdict:** Resilient (Linux).

### 2.3 Rule Database Tampering
- **Attack:** Directly modifying the SQLite database to "ALLOW" malicious traffic.
- **Defense:** Database is encrypted with **SQLCipher (AES-256)**.
- **Verdict:** Resilient.

## 3. Suggested Exploitation Scenarios
- [ ] Test bypassing rules via `ptrace` injection into a trusted process.
- [ ] Attempt to overflow the gRPC message size to crash the UI.
- [ ] Benchmark "fail-closed" behavior during early-stage boot execution.
