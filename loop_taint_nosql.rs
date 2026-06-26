use actix_web::{web, HttpResponse};
pub async fn loop_list(q: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let mut items = Vec::new();
    if let Some(v) = q.get("uid") { items.push(v.clone()); }
    let uid = items.first().cloned().unwrap_or_default();
    let doc = format!("{{"user":"{}"}}", uid); let _ = doc;
    HttpResponse::Ok().finish()
}
