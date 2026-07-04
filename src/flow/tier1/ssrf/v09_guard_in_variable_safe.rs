// v09 guard-in-variable — host allowlist in lazy static set.
use std::collections::HashSet;

fn allowed() -> HashSet<&'static str> {
    ["api.internal.example.com"].into_iter().collect()
}

pub fn fetch(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(u) = url::Url::parse(url) {
        if let Some(h) = u.host_str() {
            if allowed().contains(h) {
                let _ = u.as_str();
            }
        }
    }
    Ok(())
}
