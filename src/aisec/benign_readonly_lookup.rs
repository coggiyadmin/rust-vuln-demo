// TN — benign read-only lookup; fixed catalog by exact key.
#![allow(dead_code)]

pub fn currency_name(code: &str) -> &'static str {
    match code.to_uppercase().as_str() {
        "USD" => "US Dollar",
        "EUR" => "Euro",
        "JPY" => "Japanese Yen",
        _ => "unknown",
    }
}
