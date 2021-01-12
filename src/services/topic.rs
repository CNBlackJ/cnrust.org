#[path = "../models/topic.rs"]
mod topic;
use topic as topic_model;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Topic {
  pub id: i32,
  pub name: String,
  pub is_deleted: bool,
}

pub async fn get() -> Topic {
  let topic = Topic {
    id: 1,
    name: String::from("test topic"),
    is_deleted: false,
  };
  return topic;
}
