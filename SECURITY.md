# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability in gatekeeper, please report it responsibly.

**Preferred:** [GitHub Security Advisory](../../security/advisories/new) — creates a private channel between you and the maintainers.

**Alternative:** Email security concerns to the maintainers listed in the repository.

### What to include

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

## Response expectations

| Stage | Timeline |
|-------|----------|
| Acknowledgment | Within 48 hours |
| Initial assessment | Within 1 week |
| Fix or mitigation | Within 2 weeks for critical issues |

We will coordinate disclosure timing with you. We won't publish details until a fix is available.

## Scope

**In scope** — these are security vulnerabilities:

- **Detection bypass**: A command or URL that should trigger a rule but doesn't (false negative in a security-critical path)
- **Shell injection via hooks**: Gatekeeper's shell hooks introducing command injection vectors
- **Audit log tampering**: Ability to suppress or forge audit log entries
- **Policy bypass**: Circumventing blocklist/allowlist enforcement
- **Information disclosure**: Gatekeeper leaking sensitive data beyond the local audit log

**Not in scope**:

- False positives (non-malicious commands flagged) — file a regular bug report
- Detection of novel attack techniques not covered by existing rules — file a feature request
- Issues requiring local root/admin access — gatekeeper does not defend against privileged local attackers

## Data handling

Gatekeeper processes commands and pasted text **entirely locally**. During `check` and `paste`:

- **No network calls** are made
- **No data leaves your machine**
- Analysis results are written to a local JSONL audit log only
- Full command text is redacted in logs (first 80 chars, truncated)

The audit log lives at `~/.local/share/gatekeeper/audit.jsonl` (legacy tirith path supported). Disable with `GATEKEEPER_LOG=0` (legacy `TIRITH_LOG=0` works).

Gatekeeper has no telemetry, no analytics, no crash reporting, no phone-home behavior.

## Reproducible builds

Release artifacts are built via GitHub Actions with:
- [Sigstore cosign](https://github.com/sigstore/cosign) signatures using GitHub OIDC
- [SLSA provenance](https://slsa.dev) generation
- SHA-256 checksums for all archives

Verify a release:
```bash
cosign verify-blob --certificate-identity-regexp 'github.com/sheeki03/gatekeeper' \
  --certificate-oidc-issuer 'https://token.actions.githubusercontent.com' \
  --bundle gatekeeper-*.cosign.bundle \
  gatekeeper-*.tar.gz
```

## Supported versions

| Version | Supported |
|---------|-----------|
| 0.1.x | Yes |
