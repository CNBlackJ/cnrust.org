use bson::doc;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[path = "../util/db.rs"]
mod db;

#[derive(Debug, Serialize, Deserialize)]
pub struct Topic {
  _id: Option<ObjectId>,
  name: String,
  is_deleted: bool,
}

impl Topic {
  pub const TABLE_NAME: &'static str = "topic";
}

pub async fn get() -> Topic {
  let db_client = db::MONGO.database("test");
  for coll_name in db_client.list_collection_names(None).await? {
    println!("collection: {}", coll_name);
  }

  // let db_client = db::MONGO.database("test");
  // let coll = db_client.collection("topic");
  // coll.find(Some(doc! {}), None);

  Topic {}
}
