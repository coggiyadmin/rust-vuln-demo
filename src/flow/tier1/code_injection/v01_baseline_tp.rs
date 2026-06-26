use actix_web::{web, HttpResponse};
pub async fn code_inj(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let x = q.get("x").cloned().unwrap_or_default(); // SOURCE
    let _ = quick_js::Context::new().unwrap().eval(x.as_str()); // SINK
    HttpResponse::Ok().finish()
}
