#[macro_use]

mod endpoints;
mod managers;
mod models;
mod schema;

use actix_web::{middleware, web, App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::scope("/api/user").configure(endpoints::user::init_routes))
            .service(
                web::scope("/api/authentication").configure(endpoints::authentication::init_routes),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
