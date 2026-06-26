use actix_web::{web, HttpResponse};
pub async fn ssti_safe(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let n = q.get("n").cloned().unwrap_or_default();
    if n.len() > 32 {
        return HttpResponse::BadRequest().finish();
    }
    HttpResponse::Ok().body(format!("<p>{}</p>", n))
}
