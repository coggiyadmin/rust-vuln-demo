use actix_web::{web, HttpResponse};
pub async fn restore_safe(body: String) -> HttpResponse {
    let v: serde_json::Value = serde_json::from_str(&body).unwrap_or_default();
    HttpResponse::Ok().body(v.to_string())
}
