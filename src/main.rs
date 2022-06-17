use actix_web::{middleware, web, App, HttpServer};

mod controllers;
use controllers::topic_controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  std::env::set_var("RUST_LOG", "actix_web=debug");
  env_logger::init();

  HttpServer::new(|| {
    App::new()
      // enable logger
      .wrap(middleware::Logger::default())
      .service(
        web::scope("/topics")
        .service(topic_controller::get)
        .service(topic_controller::list)
        .service(topic_controller::create)
        .service(topic_controller::update)
        .service(topic_controller::delete)
      )
  })
  .bind("127.0.0.1:9090")?
  .run()
  .await
}
