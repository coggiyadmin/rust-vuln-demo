//! SAFE mirror — xss_probe.rs; manual HTML entity escape before body.

use actix_web::{HttpRequest, HttpResponse};

fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

pub async fn render_greeting(req: HttpRequest) -> HttpResponse {
    let name = req
        .uri()
        .query()
        .and_then(|q| q.split('=').nth(1))
        .unwrap_or("");
    let safe = escape_html(name);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!("<h1>Hello {}</h1>", safe))
}
