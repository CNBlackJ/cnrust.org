use actix_web::{middleware, web, App, HttpServer};

mod controller;

use controller::test_json;
use controller::topic;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  HttpServer::new(|| {
    App::new()
      // enable logger
      .wrap(middleware::Logger::default())
      .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
      .service(web::resource("/extractor").route(web::post().to(test_json::index)))
      .service(
        web::resource("/extractor2")
          .data(web::JsonConfig::default().limit(1024)) // <- limit size of the payload (resource level)
          .route(web::post().to(test_json::extract_item)),
      )
      .service(web::resource("/manual").route(web::post().to(test_json::index_manual)))
      .service(web::resource("/mjsonrust").route(web::post().to(test_json::index_mjsonrust)))
      .service(web::resource("/").route(web::post().to(test_json::index)))
      .service(web::resource("/topics").route(web::get().to(topic::get)))
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}
