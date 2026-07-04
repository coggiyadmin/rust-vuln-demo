// SQLi variant: sqlx-style raw query string (framework sink).
use actix_web::{web, HttpResponse};
pub async fn sqlx_raw(req: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let role = req.get("role").cloned().unwrap_or_default();
    let q = format!("SELECT * FROM users WHERE role='{}'", role); // SINK CWE-89
    let _ = q;
    HttpResponse::Ok().finish()
}
