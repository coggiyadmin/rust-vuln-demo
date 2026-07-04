// v09 guard-in-variable — host allowlist in HashSet.
use actix_web::{web, HttpResponse, Responder};
use std::collections::HashSet;

fn allowed() -> HashSet<&'static str> {
    ["app.example.com", "cdn.example.com"].into_iter().collect()
}

pub async fn go(q: web::Query<std::collections::HashMap<String, String>>) -> impl Responder {
    let nxt = q.get("next").cloned().unwrap_or_default();
    if let Ok(u) = url::Url::parse(&nxt) {
        if let Some(h) = u.host_str() {
            if allowed().contains(h) {
                return HttpResponse::Found().append_header(("Location", nxt)).finish();
            }
        }
    }
    HttpResponse::Found().append_header(("Location", "/home")).finish()
}
