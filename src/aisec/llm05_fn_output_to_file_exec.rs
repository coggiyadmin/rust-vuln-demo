// Improper Output Handling FN (OWASP LLM05) — deferred exec; output written then run later. MISS.
#![allow(dead_code)]
use std::process::Command;

const SCRIPT: &str = "/var/app/plugins/generated.sh";

pub fn stage(code: &str) -> std::io::Result<()> { std::fs::write(SCRIPT, code) } // SOURCE
pub fn activate() -> std::io::Result<std::process::Output> {                      // SINK (deferred)
    Command::new("sh").arg(SCRIPT).output()
}
