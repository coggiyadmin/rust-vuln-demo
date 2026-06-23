// Vector/Embedding FN (OWASP LLM08) — shared cache keyed only by query, not tenant. MISS.
#![allow(dead_code)]
use std::collections::HashMap;
pub fn retrieve(cache: &mut HashMap<String, Vec<String>>, tenant: &str, query: &str) -> Vec<String> {
    if let Some(hit) = cache.get(query) { return hit.clone(); } // SINK (LLM08 FN): key omits tenant
    let res = vec![format!("{}:{}", tenant, query)];
    cache.insert(query.to_string(), res.clone());
    res
}
