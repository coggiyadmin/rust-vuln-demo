// Deserialization variant — serde_yaml over untrusted input into a dynamic Value.
use actix_web::{web, HttpResponse};

pub async fn load(body: web::Bytes) -> HttpResponse {
    let s = String::from_utf8_lossy(&body); // SOURCE
    let _v: serde_yaml::Value = serde_yaml::from_str(&s).unwrap_or(serde_yaml::Value::Null); // SINK CWE-502
    HttpResponse::Ok().finish()
}
