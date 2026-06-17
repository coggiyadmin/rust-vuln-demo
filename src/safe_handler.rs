use actix_web::{HttpRequest, HttpResponse};
use std::path::{Path, PathBuf};
use std::process::Command;

/// NEGATIVE TEST FILE — secure equivalents of every vulnerable pattern.
///
/// Flows user input through safe APIs to each sink type. The scanner MUST
/// produce ZERO security findings here. Any finding is a FALSE POSITIVE.
///
/// Safe patterns exercised:
///   path_traversal    → canonicalize + starts_with prefix check
///   command_injection → Command with fixed program + arg array (no shell)
///   ssrf              → host allowlist before request
///   hardcoded creds   → std::env::var, nothing in source

const UPLOAD_ROOT: &str = "/var/app/uploads";
const ALLOWED_HOSTS: [&str; 2] = ["api.internal.example.com", "cdn.example.com"];

// SAFE path — canonicalize and verify the result stays under UPLOAD_ROOT
pub async fn read_file(req: HttpRequest) -> HttpResponse {
    let filename: &str = req.match_info().get("filename").unwrap_or("");
    let base = Path::new(UPLOAD_ROOT);
    let candidate: PathBuf = base.join(filename);

    match candidate.canonicalize() {
        Ok(resolved) if resolved.starts_with(base) => {
            match std::fs::read_to_string(&resolved) {
                Ok(contents) => HttpResponse::Ok().body(contents),
                Err(_) => HttpResponse::NotFound().finish(),
            }
        }
        _ => HttpResponse::Forbidden().finish(),
    }
}

// SAFE command — fixed program, user value is a single argv element, no shell
pub async fn run_diagnostic(req: HttpRequest) -> HttpResponse {
    let target = req.match_info().get("target").unwrap_or("");
    // No "sh -c": `dig` receives target as one argument; injection impossible
    let output = Command::new("dig").arg("+short").arg("--").arg(target).output();
    match output {
        Ok(out) => HttpResponse::Ok().body(String::from_utf8_lossy(&out.stdout).to_string()),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// SAFE ssrf — host validated against an allowlist before the request
pub async fn proxy_request(req: HttpRequest) -> HttpResponse {
    let url_str = req.uri().query().and_then(|q| q.split('=').nth(1)).unwrap_or("");
    let parsed = match url::Url::parse(url_str) {
        Ok(u) => u,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let host = parsed.host_str().unwrap_or("");
    if !ALLOWED_HOSTS.contains(&host) {
        return HttpResponse::Forbidden().finish();
    }
    match reqwest::get(parsed).await {
        Ok(r) => HttpResponse::Ok().body(r.text().await.unwrap_or_default()),
        Err(_) => HttpResponse::BadGateway().finish(),
    }
}

// SAFE config — secrets come from the environment, nothing hardcoded
pub fn database_url() -> String {
    std::env::var("DATABASE_URL").unwrap_or_default()
}
