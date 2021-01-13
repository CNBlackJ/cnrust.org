use actix_web::{Error, HttpResponse};

#[path = "../services/topic_service.rs"]
mod topic_service;

pub async fn get() -> Result<HttpResponse, Error> {
  let topic_item = topic_service::get();

  Ok(HttpResponse::Ok().json(topic_item))
}
