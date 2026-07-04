use actix_web::{web, HttpResponse};
use std::collections::HashMap;
use std::sync::Mutex;

static BTIER_SESSION: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());

/// B-tier PAT-TRUST-01 — client-supplied role stored in session map (CWE-501).
pub async fn trust_boundary_tp(req: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let sid = req.get("sid").cloned().unwrap_or_default();
    let role = req.get("role").cloned().unwrap_or_default();
    if let Ok(mut m) = BTIER_SESSION.lock() {
        m.insert(sid, role); // SINK CWE-501
    }
    HttpResponse::Ok().finish()
}
