#[macro_use]

mod endpoints;
mod models;
mod schema;
mod managers;

use actix_web::{middleware, web, App, HttpServer};


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/hello").to(|| async { "Hello world!" }))
			.service(web::scope("/user").configure(endpoints::user::user_ep::init_routes))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
