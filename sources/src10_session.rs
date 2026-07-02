use actix_web::{web, HttpResponse};
use std::process::Command;
pub async fn src01_query(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let v = q.get("q").cloned().unwrap_or_default(); // SOURCE SRC-01
    let _ = format!("<p>{}</p>", v);
    HttpResponse::Ok().finish()
}
