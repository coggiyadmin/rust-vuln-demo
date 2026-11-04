// cognium-dev#922 — a request-controlled host in format! stays SSRF
use actix_web::{web, HttpResponse, Responder};
use std::collections::HashMap;

pub async fn fetch(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let host = query.get("host").cloned().unwrap_or_default();
    let url = format!("https://{}/data", host);
    let resp = reqwest::get(&url).await;
    match resp {
        Ok(r) => HttpResponse::Ok().body(r.text().await.unwrap_or_default()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
