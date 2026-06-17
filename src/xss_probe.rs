//! FN probe — CWE-79 reflected XSS (Rust category probe).

use actix_web::{HttpRequest, HttpResponse};

pub async fn render_greeting(req: HttpRequest) -> HttpResponse {
    let name = req
        .uri()
        .query()
        .and_then(|q| q.split('=').nth(1))
        .unwrap_or("");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!("<h1>Hello {}</h1>", name))
}
