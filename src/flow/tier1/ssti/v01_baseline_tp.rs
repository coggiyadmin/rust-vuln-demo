use actix_web::{web, HttpResponse};
pub async fn ssti_tp(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let n = q.get("n").cloned().unwrap_or_default(); // SOURCE
    let tmpl = format!("<p>{}</p>", n); // SINK CWE-1336
    HttpResponse::Ok().body(tmpl)
}
