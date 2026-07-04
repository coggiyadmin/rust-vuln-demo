// v09 guard-in-variable
use std::collections::HashSet;
pub fn render(tmpl: &str) -> bool { ["Hello {{name}}"].into_iter().collect::<HashSet<&str>>().contains(tmpl) }
