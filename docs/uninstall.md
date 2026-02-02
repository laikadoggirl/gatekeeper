# Uninstall

## Remove shell hook

Remove the `eval "$(tirith init)"` line from your shell config:

| Shell | Config file |
|-------|-------------|
| zsh | `~/.zshrc` |
| bash | `~/.bashrc` |
| fish | `~/.config/fish/config.fish` |
| PowerShell | `$PROFILE` |

## Remove binary

### cargo install
```sh
cargo uninstall tirith
```

### Homebrew
```sh
brew uninstall tirith
```

### AUR
```sh
pacman -R tirith
```

### .deb
```sh
sudo dpkg -r tirith
```

### Manual / Windows
Delete the `tirith` binary from your PATH.

## Remove data

tirith stores data in XDG-compliant directories:

```sh
# Remove config (policy, allowlist, blocklist)
rm -rf ~/.config/tirith

# Remove data (audit log, receipts, materialized hooks, last_trigger)
rm -rf ~/.local/share/tirith
```

On macOS:
```sh
rm -rf ~/Library/Application\ Support/tirith
rm -rf ~/Library/Preferences/tirith
```

On Windows:
```powershell
Remove-Item -Recurse "$env:LOCALAPPDATA\tirith"
Remove-Item -Recurse "$env:APPDATA\tirith"
```
