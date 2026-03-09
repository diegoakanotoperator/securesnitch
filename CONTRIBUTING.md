# Contributing to SecureSnitch

Thank you for your interest in contributing to SecureSnitch!

## How to Contribute
1. **Fork the Repository:** Create your own copy of the project.
2. **Create a Branch:** `git checkout -b feature/your-feature-name`.
3. **Commit Your Changes:** `git commit -m "Add some feature"`.
4. **Push to the Branch:** `git push origin feature/your-feature-name`.
5. **Open a Pull Request:** Describe your changes in detail.

## Coding Standards
- **Rust:** All code must pass `cargo fmt` and `cargo clippy`.
- **Testing:** New features must include unit or integration tests in the `tests/` directory.
- **Security:** Avoid raw shell execution. Use native Rust libraries for system interaction.
- **Documentation:** Update `ARCHITECTURE.md` if you modify core data flows.

## Development Workflow
SecureSnitch uses a modular workspace:
- `daemon/`: Backend service logic.
- `ui/`: Iced frontend application.
- `proto/`: gRPC definitions.
- `ebpf_prog/`: C-based eBPF kernel modules.

Happy coding!
