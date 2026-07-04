// v10 allowlist-set — encode after HashSet membership gate.
use std::collections::HashSet;

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

pub fn greet(name: &str) -> String {
    let safe: HashSet<&str> = ["alice", "bob"].into_iter().collect();
    if safe.contains(name) {
        format!("<p>Hello {}</p>", esc(name))
    } else {
        "<p>unknown</p>".into()
    }
}
