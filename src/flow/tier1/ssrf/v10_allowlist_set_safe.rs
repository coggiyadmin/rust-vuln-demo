// v10 allowlist-set — prefix allowlist before outbound request.
pub fn fetch(url: &str) -> bool {
    const ALLOWED: &[&str] = &["https://cdn.example.com", "https://api.example.com"];
    ALLOWED.iter().any(|p| url.starts_with(p))
}
