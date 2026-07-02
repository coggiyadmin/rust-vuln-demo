// Phase-3 parameterize mirror — bound params / prepared API
use actix_web::{web, HttpResponse};
pub async fn xxe_safe(body: web::Bytes) -> HttpResponse {
    if body.len() > 65536 {
        return HttpResponse::PayloadTooLarge().finish();
    }
    HttpResponse::Ok().finish()
}
