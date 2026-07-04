// v10 allowlist-set — prefix slice gate before redirect.
use actix_web::{web, HttpResponse, Responder};

const ALLOWED: &[&str] = &["https://app.example.com/", "https://cdn.example.com/"];

pub async fn go(q: web::Query<std::collections::HashMap<String, String>>) -> impl Responder {
    let nxt = q.get("next").cloned().unwrap_or_default();
    if ALLOWED.iter().any(|p| nxt.starts_with(p)) {
        return HttpResponse::Found().append_header(("Location", nxt)).finish();
    }
    HttpResponse::Found().append_header(("Location", "/home")).finish()
}
