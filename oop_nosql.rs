// Combination #5 — OOP × NOSQL (CWE-943, Rust).
struct Query { expr: String }

impl Query {
    fn find_direct(&self) {
        let _ = format!("$where: this.user == '{}'", self.expr); // CWE-943
    }
}

pub fn oop_nosql(user: &str) {
    Query { expr: user.to_string() }.find_direct();
}
