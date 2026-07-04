// Application entry-point cmdi (TP).
use actix_web::{web, HttpResponse};
pub async fn server_cmd(req: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let q = req.get("q").cloned().unwrap_or_default(); // SOURCE
    let _ = format!("grep {}", q); // SINK CWE-78
    HttpResponse::Ok().finish()
}
