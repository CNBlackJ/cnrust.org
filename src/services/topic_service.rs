#[path = "../models/mod.rs"]
mod models;
use models::topic_model;

pub fn get() -> String {
  topic_model::get()
}
