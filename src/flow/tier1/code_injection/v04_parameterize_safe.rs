// Phase-3 parameterize mirror — bound params / prepared API
use actix_web::{web, HttpResponse};
pub async fn code_inj_safe(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let x = q.get("x").cloned().unwrap_or_default();
    if x != "daily" && x != "weekly" { return HttpResponse::Forbidden().finish(); }
    HttpResponse::Ok().finish()
}
