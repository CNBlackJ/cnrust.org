use bson::doc;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[path = "../util/db.rs"]
mod db;

#[derive(Debug, Serialize, Deserialize)]
pub struct Topic {
  _id: u32,
  name: String,
  is_deleted: bool,
}

impl Topic {
  pub const TABLE_NAME: &'static str = "topic";
}

pub fn get(id: u32) -> String {
  let db_client = db::MONGO.database("test");
  // db_client.list_collection_names(None).unwrap();

  // let db_client = db::MONGO.database("test");
  // let coll = db_client.collection("topic");
  // coll.find(Some(doc! {}), None);
  let topic = Topic {
    _id: id,
    name: String::from("test topic"),
    is_deleted: false,
  };
  String::from("get item")
}

pub fn list() -> String {
  String::from("list")
}

pub fn create() -> String {
  String::from("create ok")
}

pub fn update() -> String {
  String::from("update ok")
}

pub fn delete() -> String {
  String::from("delete ok")
}
