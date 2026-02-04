# gatekeeper fish hook
# Binds Enter to check commands before execution.

# Guard against double-loading
if set -q _GATEKEEPER_FISH_LOADED
    return
end
set -g _GATEKEEPER_FISH_LOADED 1

# Save original key bindings function BEFORE defining our new one
# This must happen before we define fish_user_key_bindings below,
# otherwise we'd copy our own function and cause infinite recursion.
if functions -q fish_user_key_bindings; and not functions -q _gatekeeper_original_fish_user_key_bindings
    functions -c fish_user_key_bindings _gatekeeper_original_fish_user_key_bindings
end

function _gatekeeper_check_command
    set -l cmd (commandline)

    # Empty input: execute normally
    if test -z "$cmd"
        commandline -f execute
        return
    end

    # Detect leading bypass env assignment for fish-incompatible syntax
    set -l bypass 0
    set -l stripped $cmd
    if string match -r '^(GATEKEEPER|TIRITH)=0(\s+|$)' -- $cmd
        set bypass 1
        set stripped (string replace -r '^(GATEKEEPER|TIRITH)=0\s*' '' -- $cmd)
        set stripped (string trim -l -- $stripped)
    end

    # Run gatekeeper check. Binary prints warnings/blocks directly to stderr.
    if test $bypass -eq 1
        # Bypass: run stripped command without gatekeeper and without echoing buffer
        set -lx GATEKEEPER 0
        history add -- "$stripped"
        commandline -r ""
        commandline -f repaint
        eval -- $stripped
        return
    end

    # Run gatekeeper check. Binary prints warnings/blocks directly to stderr.
    gatekeeper check --shell fish -- "$cmd"
    set -l rc $status

    if test $rc -eq 1
        # Block: clear the line
        commandline -r ""
        commandline -f repaint
    else
        # Allow (0) or Warn (2): execute normally
        # Warn message already printed to stderr by the binary
        commandline -f execute
    end
end

# NOTE: Only intercepts Ctrl+V paste. Right-click and middle-click paste
# bypass this check — fish does not expose a hookable paste event.
function _gatekeeper_check_paste
    # Read clipboard content
    set -l pasted (fish_clipboard_paste 2>/dev/null)

    if test -n "$pasted"
        # Check with gatekeeper paste
        echo -n "$pasted" | gatekeeper paste --shell fish
        set -l rc $status

        if test $rc -eq 1
            # Block: discard paste
            return
        end
    end

    # Allow: insert pasted content
    commandline -i -- "$pasted"
end


function fish_user_key_bindings
    # Call original user key bindings if they existed
    if functions -q _gatekeeper_original_fish_user_key_bindings
        _gatekeeper_original_fish_user_key_bindings
    end

    # Override Enter
    bind \r _gatekeeper_check_command
    bind \n _gatekeeper_check_command

    # Paste interception (Ctrl+V only)
    bind \cv _gatekeeper_check_paste
end
