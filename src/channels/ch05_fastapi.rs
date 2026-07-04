// ch05 FastAPI-style route → SSRF
pub fn fastapi_proxy(url: &str) -> String {
    format!("GET {}", url) // SINK CWE-918
}
