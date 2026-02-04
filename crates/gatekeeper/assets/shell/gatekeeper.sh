#!/usr/bin/env sh
# gatekeeper shell hook loader
# Loads the appropriate hook based on the current shell.
# Usage: eval "$(gatekeeper init)" or `. /path/to/gatekeeper.sh`

_gatekeeper_detect_shell() {
  if [ -n "$ZSH_VERSION" ]; then
    echo "zsh"
  elif [ -n "$BASH_VERSION" ]; then
    echo "bash"
  elif [ -n "$FISH_VERSION" ]; then
    echo "fish"
  elif [ -n "$PSVersionTable" ]; then
    echo "powershell"
  else
    echo "unknown"
  fi
}

_gatekeeper_dir="$(cd "$(dirname "$0")" && pwd)"

_gatekeeper_shell="$(_gatekeeper_detect_shell)"

case "$_gatekeeper_shell" in
  zsh)
    . "${_gatekeeper_dir}/lib/zsh-hook.zsh"
    ;;
  bash)
    . "${_gatekeeper_dir}/lib/bash-hook.bash"
    ;;
  fish)
    # Fish sources differently; this path is for documentation.
    # Users should: source /path/to/shell/lib/fish-hook.fish
    echo "gatekeeper: For fish, run: source ${_gatekeeper_dir}/lib/fish-hook.fish" >&2
    ;;
  *)
    # Unknown shell or PowerShell (which uses .ps1 sourcing)
    ;;
esac

unset _gatekeeper_dir _gatekeeper_shell
