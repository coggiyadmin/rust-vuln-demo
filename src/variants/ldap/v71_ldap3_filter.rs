// LDAP variant — user value concatenated into a search filter.
use actix_web::{web, HttpResponse};
use ldap3::LdapConn;
use std::collections::HashMap;

pub async fn search(q: web::Query<HashMap<String, String>>) -> HttpResponse {
    let user = q.get("user").cloned().unwrap_or_default(); // SOURCE
    if let Ok(mut ldap) = LdapConn::new("ldap://localhost") {
        let filter = format!("(uid={})", user); // SINK CWE-90
        let _ = ldap.search("ou=people", ldap3::Scope::Subtree, &filter, vec!["cn"]);
    }
    HttpResponse::Ok().finish()
}
