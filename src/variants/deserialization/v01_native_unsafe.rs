use actix_web::{web, HttpResponse};
pub async fn restore(body: String) -> HttpResponse {
    let v: serde_json::Value = serde_yaml::from_str(&body).unwrap_or_default(); // SINK CWE-502
    HttpResponse::Ok().body(v.to_string())
}
