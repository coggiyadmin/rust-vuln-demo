// ch01 GraphQL arg → SQL sink
pub fn graphql(id: &str) {
    let _ = format!("SELECT * FROM u WHERE id='{}'", id); // SOURCE ch01
}
