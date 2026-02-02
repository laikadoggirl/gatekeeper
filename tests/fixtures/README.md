# Golden Test Fixtures

TOML files defining expected behavior for tirith rules.

## Format

```toml
name = "descriptive_name"
min_milestone = 1  # 0, 1, or 2
input = "command string"
context = "exec"   # exec or paste
shell = "posix"    # posix, fish, powershell (optional, default posix)
expected_action = "block"  # allow, warn, block
expected_rules = ["rule_id"]  # expected rule IDs to fire
```
