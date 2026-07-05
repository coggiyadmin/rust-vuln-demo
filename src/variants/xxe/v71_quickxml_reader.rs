// XXE variant — quick-xml reader over untrusted body (external entities not blocked).
use actix_web::{web, HttpResponse};
use quick_xml::Reader;

pub async fn parse(body: web::Bytes) -> HttpResponse {
    let mut reader = Reader::from_reader(&body[..]); // SOURCE + SINK CWE-611
    let mut buf = Vec::new();
    let _ = reader.read_event_into(&mut buf);
    HttpResponse::Ok().finish()
}
