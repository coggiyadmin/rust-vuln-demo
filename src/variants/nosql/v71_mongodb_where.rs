// NoSQL variant — user string concatenated into a $where clause.
use actix_web::{web, HttpResponse};
use mongodb::bson::doc;
use std::collections::HashMap;

pub async fn find(q: web::Query<HashMap<String, String>>, coll: web::Data<mongodb::Collection<mongodb::bson::Document>>) -> HttpResponse {
    let name = q.get("name").cloned().unwrap_or_default(); // SOURCE
    let filter = doc! { "$where": format!("this.name == '{}'", name) }; // SINK CWE-943
    let _ = coll.find(filter, None).await;
    HttpResponse::Ok().finish()
}
