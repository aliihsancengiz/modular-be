pub mod user_ep;

use crate::models;
use actix_web::{get, post, web, HttpResponse};
use user_ep::UserEndpoint;

#[post("/login")]
async fn login(user: web::Json<models::user::UserLogin>) -> HttpResponse {
    let usr = user.into_inner();
    let mut ep = UserEndpoint::new();
    match ep.login(usr) {
        Some(resp) => HttpResponse::Ok().json(resp),
        None => HttpResponse::Unauthorized().json("Invalid Credentials."),
    }
}

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
    ep.all();
    HttpResponse::Ok().json("Hello from all.")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
    cfg.service(register);
    cfg.service(all);
}
