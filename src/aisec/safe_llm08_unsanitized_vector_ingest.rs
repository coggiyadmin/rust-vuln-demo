// SAFE mirror (OWASP LLM08) — sanitized + size-bounded per tenant.
#![allow(dead_code)]
use std::collections::HashMap;
pub fn ingest(index: &mut HashMap<String, Vec<String>>, tenant: &str, text: &str) {
    let mut clean = text.replace('\u{0}', "");
    clean.truncate(20000);
    index.entry(tenant.to_string()).or_default().push(clean);
}
