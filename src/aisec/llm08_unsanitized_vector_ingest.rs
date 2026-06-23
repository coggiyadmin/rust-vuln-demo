// Vector/Embedding Weakness (OWASP LLM08) — untrusted doc embedded unsanitized. TP.
#![allow(dead_code)]
pub fn ingest(index: &mut Vec<String>, doc: String) {
    index.push(doc); // SINK (LLM08): unsanitized into shared index
}
