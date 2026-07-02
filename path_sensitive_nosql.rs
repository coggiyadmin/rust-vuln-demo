// Combination #2 — PATH-SENSITIVITY × NOSQL (CWE-943, Rust).
use actix_web::{web, HttpResponse};

pub async fn ps_nosql_neg(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let val = q.get("q").cloned().unwrap_or_default();
    if val == "safe" {
        return HttpResponse::Ok().finish();
    }
    let _ = format!("db.users.find({{name: '{}'}}", val); // CWE-943 pattern
    HttpResponse::Ok().finish()
}
