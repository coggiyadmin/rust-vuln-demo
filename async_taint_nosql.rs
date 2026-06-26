use actix_web::{web, HttpResponse};
pub async fn async_route(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let uid = q.get("uid").cloned().unwrap_or_default();
    let uid = async { uid }.await;
    let doc = format!("{{"user":"{}"}}", uid); let _ = doc;
    HttpResponse::Ok().finish()
}
