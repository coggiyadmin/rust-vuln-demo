// Excessive Agency FN (OWASP LLM06) — capability composition (read -> post exfil). MISS.
#![allow(dead_code)]

pub fn read_note(p: &str) -> std::io::Result<String> { std::fs::read_to_string(p) } // scoped read

pub fn post_message(channel: &str, text: &str) -> String {                          // scoped post
    format!("POST https://chat.example.com/{} body={}", channel, text)
}
