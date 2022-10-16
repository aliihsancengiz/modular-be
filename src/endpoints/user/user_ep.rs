use actix_web::{post, web, HttpRequest, HttpResponse};
use crate::models;


#[post("/login")]
async fn login(user : web::Json<models::user::UserLogin>) -> HttpResponse {
	let usr = user.into_inner();
	println!("Login Request : {} {}",usr.username,usr.password);
	HttpResponse::Ok().json("Hello from login.")
}

#[post("/register")]
async fn register(user: web::Json<models::user::UserRegister>) -> HttpResponse {
	let usr = user.into_inner();
	println!("Register Request : {} {} {}",usr.username,usr.email,usr.password);
    HttpResponse::Ok().json("Hello from register.")
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
    cfg.service(register);
}
