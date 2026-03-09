# Command Execution Risk Analysis

## Status: **Medium Severity**
## Location: `daemon/core/core.go`

### Description
The Go daemon contains a centralized utility function `Exec` for executing shell commands. 

### Implementation
```go
func Exec(executable string, args []string) (string, error) {
	path, err := exec.LookPath(executable)
	if err != nil {
		return "", err
	}

	raw, err := exec.Command(path, args...).CombinedOutput()
	if err != nil {
		return "", err
	}
	return Trim(string(raw)), nil
}
```

### Risk Assessment
While the current implementation uses `exec.Command` with a slice of arguments (which prevents basic shell injection like `; rm -rf /`), it is still a powerful and potentially dangerous wrapper. 

If any caller passes data that originates from an untrusted source (like a UI-defined path or a network-provided string) into the `args` slice, an attacker could potentially pass flags that lead to arbitrary code execution or file modification (e.g., passing `--exec` or `--output` to a system utility).

### Current Usage Audit
- **Auditctl:** Uses hardcoded strings for rules. (Safe)
- **Mount:** Uses hardcoded flags and a specific mount point. (Safe)
- **Iptables:** Uses generated rules based on validated connection data. (Safe)

### Recommendation
1.  Restrict the `Exec` function to only allow a predefined whitelist of executables.
2.  Implement strict validation for all arguments passed to `Exec`.
3.  Prefer using native Go libraries (like `netlink` or `syscall`) instead of calling CLI tools.
