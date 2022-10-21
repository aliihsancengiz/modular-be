pub mod user_ep;

use crate::managers::auth::AuthorizationService;
use crate::models;
use actix_web::{
    delete, get,
    http::header::ContentType,
    post, put,
    web::{self},
    HttpResponse,
};
use user_ep::UserEndpoint;

#[post("")]
async fn registerall_user(
    user: web::Json<models::user::User>,
    _: AuthorizationService,
) -> HttpResponse {
    let mut usr = user.into_inner();
    let mut ep_manager = UserEndpoint::new();
    match ep_manager.register(&mut usr) {
        Some(resp) => HttpResponse::Ok().json(resp),
        None => HttpResponse::Conflict().json("Cannot register user."),
    }
}

#[get("")]
async fn get_all_user(_: AuthorizationService) -> HttpResponse {
    let mut ep_manager = UserEndpoint::new();
    match ep_manager.all() {
        Some(resp) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(resp),
        None => HttpResponse::Ok().json("{}"),
    }
}

#[put("")]
async fn update_user(user: web::Json<models::user::User>, _: AuthorizationService) -> HttpResponse {
    let mut ep_manager = UserEndpoint::new();
    match ep_manager.update(&mut user.into_inner()) {
        Some(resp_str) => HttpResponse::Ok().json(resp_str),
        None => HttpResponse::NotFound().finish(),
    }
}

#[delete("")]
async fn delete_user(user: web::Json<models::user::User>, _: AuthorizationService) -> HttpResponse {
    let mut ep_manager = UserEndpoint::new();
    match ep_manager.delete(user.into_inner()) {
        Some(resp_str) => HttpResponse::Ok().json(resp_str),
        None => HttpResponse::NotFound().finish(),
    }
}

fn init_database() {
    let mut usr = models::user::User {
        id: 1,
        username: "admin".to_string(),
        password: "admin".to_string(),
        email: "admin@admin.com".to_string(),
    };
    let mut ep_manager = UserEndpoint::new();
    ep_manager.register(&mut usr).unwrap_or("".to_string());
}
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    init_database();
    cfg.service(registerall_user);
    cfg.service(get_all_user);
    cfg.service(update_user);
    cfg.service(delete_user);
}
