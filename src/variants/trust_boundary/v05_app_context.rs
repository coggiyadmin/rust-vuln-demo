// trust_boundary variant: request extensions as trusted slot.
use actix_web::{web, HttpRequest, HttpResponse};
pub async fn ext(req: HttpRequest, q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let role = q.get("role").cloned().unwrap_or_default();
    req.extensions_mut().insert(role); // SINK CWE-501 trusted extension store
    HttpResponse::Ok().finish()
}
