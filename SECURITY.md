# Security Policy

## Supported Versions
Only the latest stable version of SecureSnitch is supported.

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |

## Reporting a Vulnerability
We take security seriously. If you discover a vulnerability, please report it privately via email to the maintainers or through the GitHub Security Advisory feature. Do not open public issues for zero-day vulnerabilities.

### Our Commitment
- We will acknowledge your report within 48 hours.
- We will provide an estimated timeline for a fix.
- We will coordinate a public disclosure once the remediation is verified.

## Hardening Features
SecureSnitch includes the following built-in protections:
- **Rust Memory Safety:** Prevents buffer overflows and use-after-free.
- **SQLCipher:** All firewall rules and logs are encrypted at rest.
- **Litebox Enclave:** Protects the policy decision logic from in-memory tampering.
