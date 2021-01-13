#[path = "../models/topic_model.rs"]
mod topic_model;

pub fn get() -> topic_model::Topic {
  let topic = topic_model::get();
  topic
}
