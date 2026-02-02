# Roadmap

## Phase 2 Backlog

These items are candidates for future development. Priority and scope may change based on dogfooding feedback.

### Interactive confirmation for strict_warn
- Hook-mediated Y/N prompt for WARN-level findings in interactive mode
- Allows users to acknowledge warnings without blocking

### Socket/daemon mode
- Persistent background process to avoid cold-start latency
- Shell hooks connect via Unix socket instead of spawning a process
- Target: <0.5ms hook latency

### Network-aware checks
- Resolve shortened URLs to final destination before analysis
- Check certificate transparency logs for suspicious domains
- DNS-based threat intelligence lookups

### `tirith scan` CI mode
- Scan a file or directory for URLs and analyze them
- Exit code reflects highest severity found
- SARIF output for GitHub Code Scanning integration

### `tirith policy validate`
- Validate policy YAML syntax and semantics
- Check for conflicting allowlist/blocklist entries
- Warn about deprecated or unknown fields

### `tirith explain --rule <id>`
- Show detailed rationale for a specific rule
- Include examples of what it catches and why
- Link to relevant threat documentation

### IDE integration
- VS Code extension that analyzes terminal commands
- Inline warnings for URLs in code and configuration files

### Custom rule authoring SDK
- YAML or Lua-based rule definitions
- User-defined patterns with custom severity and evidence
- Rule testing framework
