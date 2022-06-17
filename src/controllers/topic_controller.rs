use actix_web::{web, get, post, put, delete, HttpResponse, Responder};

#[path = "../services/topic_service.rs"]
mod topic_service;

#[get("/{id}")]
pub async fn get(path: web::Path<(u32,)>) -> impl Responder{
  let topic_item = topic_service::get(path.into_inner().0);

  HttpResponse::Ok().json(topic_item)
}

#[get("")]
pub async fn list() -> impl Responder {
  let topic_list = topic_service::list();

  HttpResponse::Ok().json(topic_list)
}

#[post("")]
pub async fn create() -> impl Responder {
  let topic_item = topic_service::create();

  HttpResponse::Ok().json(topic_item)
}

#[put("")]
pub async fn update() -> impl Responder {
  let topic_item = topic_service::update();

  HttpResponse::Ok().json(topic_item)
}

#[delete("/{id}")]
pub async fn delete() -> impl Responder {
  let topic_item = topic_service::delete();

  HttpResponse::Ok().json(topic_item)
}

