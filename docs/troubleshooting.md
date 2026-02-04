# Troubleshooting

## Shell hooks not loading

Run `gatekeeper doctor` to see the hook directory being used and whether hooks were materialized from the embedded binary.

If hooks are not found:
1. Ensure `gatekeeper` is in your PATH
2. Run `eval "$(gatekeeper init)"` and check for error messages
3. Set `GATEKEEPER_SHELL_DIR` (legacy `TIRITH_SHELL_DIR` also works) to point to your shell hooks directory explicitly

## Bash: Enter mode vs preexec mode

gatekeeper supports two bash integration modes:
- **enter mode** (default): Binds to Enter key via `bind -x`. Intercepts commands before execution.
- **preexec mode**: Uses `DEBUG` trap. Compatible with more environments but slightly different behavior.

Set via: `export GATEKEEPER_BASH_MODE=enter` or `export GATEKEEPER_BASH_MODE=preexec` (legacy TIRITH_BASH_MODE supported)

## PowerShell: PSReadLine conflicts

If using PSReadLine, ensure the gatekeeper hook loads after PSReadLine initialization. The hook overrides `PSConsoleHostReadLine` to intercept pastes.

## Latency

gatekeeper's Tier 1 fast path (no URLs detected) targets <2ms. If you notice latency:

1. Run `gatekeeper check --json -- "your command"` and check `timings_ms`
2. If Tier 1 is slow, check for extremely long command strings
3. Policy file loading (Tier 2) adds ~1ms. Use `gatekeeper doctor` to see policy paths

## False positives

If a command is incorrectly blocked or warned:
1. Run `gatekeeper why` to see which rule triggered
2. Add the URL to your allowlist: `~/.config/gatekeeper/allowlist` (legacy `~/.config/tirith/allowlist` also read)
3. Override the rule severity in policy.yaml: `severity_overrides: { rule_id: LOW }` (policy discovery prefers .gatekeeper, with legacy .tirith fallback)

## Policy discovery

gatekeeper searches for policy in this order (legacy tirith paths are used only if gatekeeper paths are missing):
1. `GATEKEEPER_POLICY_ROOT` env var → `$GATEKEEPER_POLICY_ROOT/.gatekeeper/policy.yaml` (or `.yml`) (fallback: TIRITH_POLICY_ROOT)
2. Walk up from CWD looking for `.gatekeeper/policy.yaml` (or `.yml`) (fallback: .tirith)
3. `~/.config/gatekeeper/policy.yaml` (or `.yml`) (user-level; fallback: ~/.config/tirith)

Use `gatekeeper doctor` to see which policy files are active.

## Audit log location

Default: `~/.local/share/gatekeeper/log.jsonl` (XDG-compliant; legacy tirith path falls back if gatekeeper dir missing)

Each entry is a JSON line with timestamp, action, rule IDs, and redacted command.
