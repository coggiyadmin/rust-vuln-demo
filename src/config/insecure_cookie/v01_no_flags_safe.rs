use actix_web::HttpResponse;
pub fn set_session(sid: &str) -> HttpResponse {
    HttpResponse::Ok()
        .cookie(actix_web::cookie::Cookie::build("SESSIONID", sid).secure(true).http_only(true)
            .same_site(actix_web::cookie::SameSite::Lax).finish())
        .finish()
}
