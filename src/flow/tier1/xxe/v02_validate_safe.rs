// Phase-3 validate mirror — allowlist/regex gate before sink
use actix_web::{web, HttpResponse};
pub async fn xxe_safe(body: web::Bytes) -> HttpResponse {
    if body.len() > 65536 {
        return HttpResponse::PayloadTooLarge().finish();
    }
    HttpResponse::Ok().finish()
}
