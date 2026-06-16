use actix_web::{web, HttpRequest, HttpResponse};
use std::process::Command;
use std::path::Path;
use std::fs;

// CWE-798: hardcoded production secrets
const AWS_ACCESS_KEY: &str     = "AKIAIOSFODNN7EXAMPLE2";
const AWS_SECRET_KEY: &str     = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY2";
const STRIPE_SECRET: &str      = "sk_live_51ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnop12345";
const GITHUB_PAT: &str         = "ghp_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefgh01";
const ANTHROPIC_KEY: &str      = "sk-ant-api03-ABCDEFGHIJKLMNOPQRSTUVWXYZ-abcdefghijklmnopqr-STUVWX";
const DATABASE_URL: &str       = "postgresql://admin:Pr0dP@ss2024!@prod.example.com:5432/app";

// CWE-22: Path Traversal — user-controlled path with no canonicalization
pub async fn read_file(req: HttpRequest) -> HttpResponse {
    let filename: &str = req.match_info().get("filename").unwrap_or("");
    let base = Path::new("/var/app/uploads");
    let full_path = base.join(filename);           // CWE-22: ../../../etc/passwd traversal

    match fs::read_to_string(&full_path) {
        Ok(contents) => HttpResponse::Ok().body(contents),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

// CWE-22: Path traversal via query parameter
pub async fn serve_template(req: HttpRequest) -> HttpResponse {
    let template = req.uri().query()
        .and_then(|q| q.split('=').nth(1))
        .unwrap_or("index.html");

    let path = Path::new("/var/app/templates").join(template);  // CWE-22
    match fs::read_to_string(path) {
        Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

// CWE-78: Command Injection — user-controlled argument passed to shell
pub async fn run_diagnostic(req: HttpRequest) -> HttpResponse {
    let target = req.match_info().get("target").unwrap_or("");
    // Attacker sends target=; cat /etc/shadow > /tmp/out
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("dig +short {}", target))     // CWE-78: injection via target
        .output();

    match output {
        Ok(out) => HttpResponse::Ok().body(String::from_utf8_lossy(&out.stdout).to_string()),
        Err(e)  => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// CWE-78: OS command injection via header value
pub async fn process_webhook(req: HttpRequest) -> HttpResponse {
    let event_type = req.headers()
        .get("X-Event-Type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");

    let _ = Command::new("sh")
        .arg("-c")
        .arg(format!("./handlers/{}.sh", event_type))  // CWE-78: header injection
        .spawn();

    HttpResponse::Ok().finish()
}

// CWE-918: SSRF — user-controlled outbound URL
pub async fn proxy_request(req: HttpRequest) -> HttpResponse {
    let url = req.uri().query()
        .and_then(|q| q.split('=').nth(1))
        .unwrap_or("");

    // Attacker sends ?url=http://169.254.169.254/latest/meta-data/
    let response = reqwest::get(url).await;         // CWE-918: SSRF
    match response {
        Ok(r) => HttpResponse::Ok().body(r.text().await.unwrap_or_default()),
        Err(_) => HttpResponse::BadGateway().finish(),
    }
}
