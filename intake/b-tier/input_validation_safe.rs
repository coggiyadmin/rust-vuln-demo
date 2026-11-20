use actix_web::{web, HttpResponse};

const MAX: i64 = 1000;

/// Safe mirror — PAT-INPUT-01
pub async fn input_validation_safe(req: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let amount: i64 = req.get("amount").and_then(|s| s.parse().ok()).unwrap_or(0);
    if amount <= 0 || amount > MAX {
        return HttpResponse::BadRequest().body("bad amount");
    }
    HttpResponse::Ok().body("xfer ok")
}
