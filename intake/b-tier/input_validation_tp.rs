use actix_web::{web, HttpResponse};

/// B-tier PAT-INPUT-01 — unbounded transfer amount (CWE-20).
pub async fn input_validation_tp(req: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let amount = req.get("amount").cloned().unwrap_or_default(); // SINK CWE-20
    let to = req.get("to").cloned().unwrap_or_default();
    HttpResponse::Ok().body(format!("xfer {amount} to {to}"))
}
