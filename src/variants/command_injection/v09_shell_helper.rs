// Cmdi variant: shell helper via sh -c concat.
use actix_web::{web, HttpResponse};
use std::process::Command;
pub async fn shell_helper(req: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let dir = req.get("dir").cloned().unwrap_or_default();
    let _ = Command::new("sh").args(["-c", &format!("ls {}", dir)]).spawn(); // SINK CWE-78
    HttpResponse::Ok().finish()
}
