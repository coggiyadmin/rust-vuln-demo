// Improper Output Handling (OWASP LLM05) — model output run as a shell command. TP.
#![allow(dead_code)]
use std::process::Command;

pub fn run_generated(model_output: &str) -> std::io::Result<std::process::Output> {
    // SINK (LLM05): model output executed by the shell
    Command::new("sh").arg("-c").arg(model_output).output()
}
