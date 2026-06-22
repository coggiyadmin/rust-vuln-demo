//! SAFE mirror — parse into a fixed typed struct (no arbitrary type). ZERO findings.
use actix_web::{web, HttpResponse};
#[derive(serde::Deserialize, Default)]
struct Dto { name: String }
pub async fn restore(body: String) -> HttpResponse {
    let d: Dto = serde_json::from_str(&body).unwrap_or_default();
    HttpResponse::Ok().body(d.name)
}
