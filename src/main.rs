use actix_web::{middleware, web, App, HttpServer};

mod controller;

use controller::topic_controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  HttpServer::new(|| {
    App::new()
      // enable logger
      .wrap(middleware::Logger::default())
      .service(web::resource("/topics").route(web::get().to(topic_controller::get)))
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}
