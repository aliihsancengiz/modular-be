pub mod authentication;

use crate::models;
use actix_web::{post, web, HttpResponse};
use authentication::AuthEndpoint;

#[post("")]
async fn auth(user: web::Json<models::auth::UserLoginRequest>) -> HttpResponse {
    let usr = user.into_inner();
    let mut ep = AuthEndpoint::new();
    match ep.auth(usr) {
        Some(resp) => HttpResponse::Ok().json(resp),
        None => HttpResponse::Unauthorized().json("Invalid Credentials."),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(auth);
}
