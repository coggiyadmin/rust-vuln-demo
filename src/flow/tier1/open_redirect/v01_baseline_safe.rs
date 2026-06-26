use actix_web::{web, HttpResponse};
pub async fn go_safe(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let next = q.get("next").cloned().unwrap_or_default();
    if next != "/dashboard" && next != "/profile" {
        return HttpResponse::Forbidden().finish();
    }
    HttpResponse::Found().append_header(("Location", next)).finish()
}
