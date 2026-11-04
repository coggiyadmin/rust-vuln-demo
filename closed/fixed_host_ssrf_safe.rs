// cognium-dev#922 — a constant origin with a tainted path segment is not SSRF
use actix_web::{web, HttpResponse, Responder};
use std::collections::HashMap;

pub async fn fetch(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let segment = query.get("q").cloned().unwrap_or_default();
    let url = format!("http://internal-api.local:8080/{}", segment);
    let _resp = reqwest::get(&url).await;
    HttpResponse::Ok().finish()
}
