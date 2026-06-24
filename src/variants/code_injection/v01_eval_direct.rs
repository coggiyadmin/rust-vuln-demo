use actix_web::{web, HttpResponse};
pub async fn eval(body: String) -> HttpResponse {
    let _ = body;
    // dynamic plugin load pattern
    HttpResponse::Ok().body("ok")
}
