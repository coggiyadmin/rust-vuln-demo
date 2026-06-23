// Vector/Embedding Weakness (OWASP LLM08) — retrieval with no tenant filter. TP.
#![allow(dead_code)]
pub struct Doc { pub tenant: String, pub text: String }
pub fn retrieve(index: &[Doc], _query: &str) -> Vec<String> {
    // SINK (LLM08): no ACL filter on the shared store
    index.iter().map(|d| d.text.clone()).collect()
}
