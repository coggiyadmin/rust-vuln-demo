use actix_web::HttpResponse;
pub fn set_session(sid: &str) -> HttpResponse {
    HttpResponse::Ok()
        .cookie(actix_web::cookie::Cookie::build("SESSIONID", sid).secure(false).http_only(false).finish())
        .finish()
}
