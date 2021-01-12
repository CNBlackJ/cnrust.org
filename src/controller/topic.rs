use actix_web::HttpResponse;

#[path = "../services/topic.rs"]
mod topic;
use topic as topic_service;

pub async fn get() -> HttpResponse {
  let topic_item = topic_service::get();

  HttpResponse::Ok().json("aa")
}
