// SSTI variant — Tera one-off render of a user-supplied template string.
use actix_web::{web, HttpResponse};
use tera::{Tera, Context};

pub async fn render(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let tpl = q.get("tpl").cloned().unwrap_or_default(); // SOURCE
    let out = Tera::one_off(&tpl, &Context::new(), true).unwrap_or_default(); // SINK CWE-94
    HttpResponse::Ok().body(out)
}
