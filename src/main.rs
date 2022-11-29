use actix_web::{middleware, App, HttpServer};
use env_logger;
use std::env;

#[path = "./endpoints/pix.rs"]
mod pix;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(pix::hello)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
