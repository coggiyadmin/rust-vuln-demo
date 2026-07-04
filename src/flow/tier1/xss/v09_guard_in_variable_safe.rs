// v09 guard-in-variable — tag allowlist held in static set.
use std::collections::HashSet;

fn allowed_tags() -> HashSet<&'static str> {
    ["p", "span"].into_iter().collect()
}

pub fn render(tag: &str, text: &str) -> String {
    if allowed_tags().contains(tag) {
        format!("<{tag}>{text}</{tag}>")
    } else {
        "<p>blocked</p>".into()
    }
}
