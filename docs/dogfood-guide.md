# Dogfooding Guide

## Setup

1. Install gatekeeper:
   ```sh
   cargo install --path crates/gatekeeper
   ```

2. Activate in your shell:
   ```sh
   # Add to your shell config (~/.zshrc, ~/.bashrc, etc.)
   eval "$(gatekeeper init)"
   ```

3. Restart your shell or source the config.

## Verify installation

```sh
gatekeeper doctor
```

Confirm:
- Hook dir is found
- Shell is detected correctly
- Data dir exists

## Test detection

These commands should trigger findings:

```sh
# Should BLOCK (curl pipe bash)
curl https://example.com/install.sh | bash

# Should WARN (shortened URL)
curl https://bit.ly/abc123

# Should be SILENT (normal command)
ls -la
```

## Daily use

Use gatekeeper as your normal shell for at least a week. Pay attention to:

- **False positives**: Legitimate commands being flagged
- **False negatives**: Suspicious commands not being flagged
- **Latency**: Any noticeable delay when pressing Enter
- **Shell integration**: Any issues with your shell workflow

## Check audit log

```sh
# View recent entries
tail -5 ~/.local/share/gatekeeper/log.jsonl | python3 -m json.tool
```

## Customize policy

If you encounter false positives, add URLs to your allowlist:

```sh
echo "example.com" >> ~/.config/gatekeeper/allowlist
```

Or adjust severity in policy:

```yaml
# ~/.config/gatekeeper/policy.yaml
severity_overrides:
  shortened_url: LOW
```

## Report issues

1. Run `gatekeeper doctor` and save the output
2. Note the command that caused the issue (redact sensitive parts)
3. File an issue using the dogfood report template

## Uninstall

See [docs/uninstall.md](uninstall.md).
