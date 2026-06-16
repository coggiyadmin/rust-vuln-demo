//! FALSE-NEGATIVE PROBE (Rust/actix-web) — every handler is a REAL vulnerability
//! routing an actix request source into a command/path/SSRF sink. Any handler
//! that produces NO finding is a FALSE NEGATIVE.

use actix_web::{web, HttpRequest, HttpResponse};
use std::process::Command;
use std::fs;

// 1. CWE-78 — match_info path segment → shell
pub async fn cmd_matchinfo(req: HttpRequest) -> HttpResponse {
    let target = req.match_info().get("target").unwrap_or("");
    let out = Command::new("sh").arg("-c").arg(format!("dig {}", target)).output();
    HttpResponse::Ok().body(String::from_utf8_lossy(&out.unwrap().stdout).to_string())
}

// 2. CWE-22 — query string → file read
pub async fn read_query(req: HttpRequest) -> HttpResponse {
    let file = req.uri().query().unwrap_or("");
    let data = fs::read_to_string(format!("/var/app/{}", file)).unwrap_or_default();
    HttpResponse::Ok().body(data)
}

// 3. CWE-78 — web::Path extractor → command (idiomatic actix source)
pub async fn cmd_path(info: web::Path<String>) -> HttpResponse {
    let host = info.into_inner();
    let _ = Command::new("ping").arg("-c").arg("3").arg(host).output();
    HttpResponse::Ok().finish()
}

// 4. CWE-918 — web::Query extractor → outbound request
pub async fn ssrf_query(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let url = q.get("url").cloned().unwrap_or_default();
    let body = reqwest::blocking::get(&url).unwrap().text().unwrap_or_default();
    HttpResponse::Ok().body(body)
}

// 5. CWE-22 — web::Form body field → file path
pub async fn write_form(form: web::Form<std::collections::HashMap<String, String>>) -> HttpResponse {
    let name = form.get("name").cloned().unwrap_or_default();
    let _ = fs::write(format!("/var/app/uploads/{}", name), "x");
    HttpResponse::Ok().finish()
}
