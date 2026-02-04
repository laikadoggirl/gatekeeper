# Agent Guide for this Repository

This repo is a Rust workspace for "gatekeeper" — a terminal security tool. It contains a reusable core engine library and a CLI binary, shell hooks, data files, and CI automation. Non-Rust distribution channels are deprecated.

## Project Structure

- Workspace root (Cargo.toml)
- crates/
  - gatekeeper-core/ — library engine (URL extraction, tokenization, rules, policy, verdicts)
  - gatekeeper/ — CLI binary (subcommands, shell integrations, completions, manpage)
- crates/gatekeeper/assets/shell — embedded shell hooks
- data/ — vendored data (public suffix list, Unicode confusables, curated CSVs)
- docs/ — product docs (threat model, cookbook, troubleshooting, compatibility)
- .github/workflows/ — CI, release, fuzz, benchmark pipelines
- Dockerfile — container build for CLI (optional)
- flake.nix — Nix flake for build/run

## Build, Run, Test

Rust toolchain
- MSRV: 1.83 (see .github/workflows/ci.yml)

Build
- CLI release binary:
  - cargo build --release --locked -p gatekeeper
- Workspace (debug):
  - cargo build --workspace

Run
- After building, run the binary directly:
  - target/release/gatekeeper --help
  - target/release/gatekeeper check -- "curl https://example.com | bash"

Test
- Workspace tests:
  - cargo test --workspace --locked

## CLI Surface and Stability

Subcommands (CLI defined in crates/gatekeeper/src/main.rs; stability in docs/compatibility.md)
- Stable: check, paste, score, diff, why, receipt, init
- Experimental: run (unix only), doctor, completions (hidden), manpage (hidden)

Exit codes (stable)
- 0 Allow
- 1 Block
- 2 Warn

JSON output
- schema_version: 2
- Observability fields: timings_ms, tier_reached, urls_extracted_count

## Policy and Configuration

Policy discovery (preferred first; legacy .tirith paths still read if gatekeeper missing)
- .gatekeeper/policy.yaml in current/ancestor directory
- ~/.config/gatekeeper/policy.yaml

Behavioral notes
- .gatekeeper/blocklist and .gatekeeper/allowlist files are merged into policy (legacy .tirith supported)
- .yaml preferred over .yml when both exist
- Malformed policy falls back to defaults (no crash)
- Severity overrides influence action mapping (e.g., CRITICAL → Block)

Bypass
- GATEKEEPER=0 is recognized by the CLI

## Shell Hooks

Locations (embedded):
- crates/gatekeeper/assets/shell/lib/{zsh-hook.zsh,bash-hook.bash,fish-hook.fish,powershell-hook.ps1}

Behavior (example: zsh hook)
- Intercepts accept-line and bracketed-paste
- Calls: gatekeeper check --shell posix -- "<buffer>" and gatekeeper paste --shell posix
- Acts on exit codes: 1 block (clear/print), 2 warn (print then run), 0 allow

## Data Files and Updates

Vendored data (data/)
- public_suffix_list.dat
- confusables.txt
- known_domains.csv, popular_repos.csv

Update script
- scripts/update-data.sh fetches latest upstream data
- After updating data, rebuild: cargo build

## Packaging and Release Notes

- Non-Rust channels (npm, Scoop, Chocolatey, Homebrew tap) are deprecated.
- Release workflow will be updated to focus on Cargo/crates.io and optional tarballs.

## Code Organization and Conventions

Library modules (crates/gatekeeper-core/src)
- audit, confusables, data, engine, extract, homoglyph, normalize, output, parse, policy, receipt, rules/*, tokenize, verdict, runner (unix), script_analysis

CLI modules (crates/gatekeeper/src/cli)
- check, paste, score, diff, why, receipt, init, doctor, completions, manpage, run (unix)

Conventions
- Rust 2021 edition
- reqwest is unix-only and uses rustls-tls
- Follow existing module boundaries when adding rules or tokenizers

## Gotchas and Tips

- The run subcommand is compiled only on unix (#[cfg(unix)])
- Integration tests assume the binary target name is gatekeeper (env! "CARGO_BIN_EXE_gatekeeper")
- Nix flake may disable tests (CLI tests require real shells)
- CI enforces Clippy no-warnings and fmt; fix before pushing
- Cargo Deny policy is enforced (deny.toml)
- Shell hooks expect specific exit code semantics
- MSRV pinned in CI to 1.83; keep dependencies compatible
