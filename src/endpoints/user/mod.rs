pub mod user_ep;

use crate::models;
use actix_web::{http::header::ContentType, get, post, web, HttpResponse};
use user_ep::UserEndpoint;

#[post("/register")]
async fn register(user: web::Json<models::user::UserRegister>) -> HttpResponse {
    let mut usr = user.into_inner();
    let mut ep = UserEndpoint::new();
    match ep.register(&mut usr) {
        Some(resp) => HttpResponse::Ok().json(resp),
        None => HttpResponse::Conflict().json("Cannot register user."),
    }
}

#[get("/all")]
async fn all() -> HttpResponse {
    let mut ep = UserEndpoint::new();
    match ep.all() {
		Some(resp) => HttpResponse::Ok().content_type(ContentType::json()).json(resp),
		None =>  HttpResponse::Ok().json("{}"),
	}
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
    cfg.service(all);
}
