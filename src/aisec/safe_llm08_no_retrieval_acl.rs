// SAFE mirror (OWASP LLM08) — retrieval scoped to the caller's tenant first.
#![allow(dead_code)]
pub struct Doc { pub tenant: String, pub text: String }
pub fn retrieve(index: &[Doc], tenant: &str, _query: &str) -> Vec<String> {
    index.iter().filter(|d| d.tenant == tenant).map(|d| d.text.clone()).collect()
}
