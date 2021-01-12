use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Topic {
  pub id: i32,
  pub name: String,
  pub is_deleted: bool,
}
