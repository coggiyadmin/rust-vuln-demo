// Provenance/Misinfo (OWASP LLM09) — hallucinated dependency (slopsquatting surface).
#![allow(dead_code)]
use hallucinated_http_retry::Client; // SINK (LLM09): fabricated crate

pub fn fetch_url(url: &str) -> String { Client::new().get(url) }
