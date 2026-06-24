// FP-target (#162/#140) — CLI profile. A dev tool that execs an argv arg.
#![allow(dead_code)]
pub fn run() {
    let arg = std::env::args().nth(1).unwrap_or_default();
    let _ = std::process::Command::new("sh").arg("-c").arg(arg).output(); // operator-controlled
}
