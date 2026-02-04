# Uninstall

## Remove shell hook

Remove the `eval "$(gatekeeper init)"` line from your shell config:

| Shell | Config file |
|-------|-------------|
| zsh | `~/.zshrc` |
| bash | `~/.bashrc` |
| fish | `~/.config/fish/config.fish` |
| PowerShell | `$PROFILE` |

## Remove binary



### Cargo
```sh
cargo uninstall gatekeeper
```











### Manual
Delete the `gatekeeper` binary from your PATH.

## Remove data

gatekeeper stores data in XDG-compliant directories (legacy tirith paths may also exist):

```sh
# Remove config (policy, allowlist, blocklist)
rm -rf ~/.config/gatekeeper
rm -rf ~/.config/tirith

# Remove data (audit log, receipts, materialized hooks, last_trigger)
rm -rf ~/.local/share/gatekeeper
rm -rf ~/.local/share/tirith
```

On macOS:
```sh
rm -rf ~/Library/Application\ Support/gatekeeper
rm -rf ~/Library/Preferences/gatekeeper
rm -rf ~/Library/Application\ Support/tirith
rm -rf ~/Library/Preferences/tirith
```

On Windows:
```powershell
Remove-Item -Recurse "$env:LOCALAPPDATA\gatekeeper"
Remove-Item -Recurse "$env:APPDATA\gatekeeper"
Remove-Item -Recurse "$env:LOCALAPPDATA\tirith"
Remove-Item -Recurse "$env:APPDATA\tirith"
```
