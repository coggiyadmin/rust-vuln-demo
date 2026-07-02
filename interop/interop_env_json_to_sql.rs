// IL-49 — JSON env blob → SQL string (IL-5 config frontier).
pub fn lookup(user_filter: &str) {
    let filt = user_filter;
    let _q = format!("SELECT * FROM u WHERE n='{}'", filt);
}
