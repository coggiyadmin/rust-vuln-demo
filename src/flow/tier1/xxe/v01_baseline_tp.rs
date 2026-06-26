use actix_web::{web, HttpResponse};
pub async fn xxe_tp(body: web::Bytes) -> HttpResponse {
    let raw = String::from_utf8_lossy(&body).to_string(); // SOURCE
    let _ = raw.parse::<()>(); // SINK CWE-611 stub — external entity parse
    HttpResponse::Ok().finish()
}
