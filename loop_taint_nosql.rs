// Combination #3 — LOOP × NOSQL (CWE-943, Rust).
pub fn loop_nosql(items: &[String]) {
    for x in items {
        let _ = format!("find({{user: '{}'}}", x); // CWE-943
    }
}
