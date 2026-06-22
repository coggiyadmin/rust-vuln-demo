//! SAFE mirror — redirect resolved to a constant literal. ZERO findings expected.
use actix_web::{web, HttpResponse};
pub async fn go(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let next = q.get("next").cloned().unwrap_or_default();
    let dest = match next.as_str() { "account" => "/account", "help" => "/help", _ => "/home" };
    HttpResponse::Found().append_header(("Location", dest)).finish()
}
