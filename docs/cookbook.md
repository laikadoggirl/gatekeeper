# Policy Cookbook

## 1. Strict Organization (Fail Closed, No Bypass)

```yaml
# .tirith/policy.yaml (repo root)
fail_mode: closed
allow_bypass: false
severity_overrides:
  shortened_url: HIGH
  plain_http_to_sink: CRITICAL
```

All findings block execution. No bypass mechanism. Shortened URLs and plain HTTP are escalated.

## 2. Personal Developer (Defaults + Allowlist)

```yaml
# ~/.config/tirith/policy.yaml
fail_mode: open
allow_bypass: true
```

With allowlist at `~/.config/tirith/allowlist`:
```
raw.githubusercontent.com
homebrew.bintray.com
get.docker.com
```

Default severity mappings. Allowlisted URLs skip analysis.

## 3. CI Safe Mode (Non-Interactive, JSON Output)

```bash
# In CI pipeline
tirith check --non-interactive --json -- curl https://example.com/setup.sh | bash
EXIT=$?
if [ $EXIT -eq 1 ]; then
  echo "BLOCKED by tirith" >&2
  exit 1
fi
```

Non-interactive mode never prompts. JSON output for machine parsing.

## 4. Docker-Focused (Escalate Docker Rules)

```yaml
# .tirith/policy.yaml
severity_overrides:
  docker_untrusted_registry: CRITICAL
  docker_tag_latest: HIGH
```

All Docker-related findings are escalated. Other rules use default severity.

## 5. Learning Mode (All Low Severity)

```yaml
# ~/.config/tirith/policy.yaml
fail_mode: open
allow_bypass: true
severity_overrides:
  curl_pipe_shell: LOW
  wget_pipe_shell: LOW
  pipe_to_interpreter: LOW
  punycode_domain: LOW
  confusable_domain: LOW
```

Everything becomes a LOW-severity warning. Nothing blocks. Useful for onboarding.
