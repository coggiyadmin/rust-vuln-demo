// Library-API command param — FP-target (#128).
pub fn run_cmd(cmd: &str) -> String {
    format!("sh -c {}", cmd)
}
