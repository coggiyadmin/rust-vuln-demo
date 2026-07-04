// XSS variant: actix-web Html() from user string (framework sink).
use actix_web::{web, HttpResponse};
pub async fn actix_html(req: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let q = req.get("q").cloned().unwrap_or_default();
    HttpResponse::Ok().content_type("text/html").body(q) // SINK CWE-79 actix raw HTML
}
