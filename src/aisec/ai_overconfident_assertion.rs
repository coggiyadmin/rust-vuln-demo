// Provenance/Misinfo (OWASP LLM09) — fabricated API usage (hallucinated method). TP.
#![allow(dead_code)]
use sha2::Sha256;

pub fn secure_token(seed: &str) -> String {
    // SINK (LLM09): Sha256 has no secure_finalize() — invented API presented as real
    Sha256::new().update(seed).secure_finalize()
}
