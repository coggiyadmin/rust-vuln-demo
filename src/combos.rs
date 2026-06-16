//! Combination × language matrix — RUST row.
//! Uses std::env::args() (a recognized cli_arg source — avoids the actix gap #73)
//! so each combination itself is what's measured. FN probes must FIRE.

use std::process::Command;

// #2 path-sensitivity — tainted used in the failure branch
pub fn c2_path() {
    let x = std::env::args().nth(1).unwrap_or_default();
    if x.parse::<i64>().is_err() {                 // validation fails...
        Command::new("sh").arg("-c").arg(format!("echo {}", x)).status().ok(); // ...used anyway CWE-78
    }
}

// #3 loop-carried — iterate over tainted args
pub fn c3_loop() {
    for a in std::env::args().skip(1) {
        Command::new("sh").arg("-c").arg(format!("echo {}", a)).status().ok(); // CWE-78
    }
}

// #5 OOP/struct flow — field set from input, used in a method
struct Job { cmd: String }
impl Job {
    fn run(&self) { Command::new("sh").arg("-c").arg(&self.cmd).status().ok(); } // CWE-78
}
pub fn c5_oop() {
    let j = Job { cmd: std::env::args().nth(1).unwrap_or_default() };
    j.run();
}

// #11 fan-out — one source → two sinks
pub fn c11_fanout() {
    let u = std::env::args().nth(1).unwrap_or_default();
    Command::new("sh").arg("-c").arg(&u).status().ok();               // sink 1
    let _ = std::fs::read_to_string(format!("/var/app/{}", u));        // sink 2 (path)
}

// #7 fake sanitizer — no-op named clean()
fn clean(x: String) -> String { x }
pub fn c7_fake() {
    let u = clean(std::env::args().nth(1).unwrap_or_default());
    Command::new("sh").arg("-c").arg(&u).status().ok();               // CWE-78
}

// #9 comment/string-literal — sink syntax only in a string literal (FP: stay clean)
pub fn c9_string() -> String {
    // Command::new("sh").arg(user)   <- comment, must not fire
    String::from("Command::new(\"sh\").arg(user_input)")              // literal, must not fire
}
