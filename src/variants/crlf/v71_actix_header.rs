// CRLF variant — user value reflected into a response header.
use actix_web::{web, HttpResponse};
use std::collections::HashMap;

pub async fn header(q: web::Query<HashMap<String, String>>) -> HttpResponse {
    let lang = q.get("lang").cloned().unwrap_or_default(); // SOURCE
    HttpResponse::Ok().append_header(("Content-Language", lang)).finish() // SINK CWE-113
}
