#[path = "../models/mod.rs"]
mod models;
use models::topic_model;

pub fn get(id: u32) -> String {
  topic_model::get(id)
}

pub fn list() -> String {
  topic_model::list()
}

pub fn create() -> String {
  topic_model::create()
}

pub fn update() -> String {
  topic_model::update()
}

pub fn delete() -> String {
  topic_model::delete()
}
