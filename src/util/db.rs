use lazy_static::lazy_static;
use mongodb::{options, Client};

lazy_static! {
  pub static ref MONGO: Client = create_mongo_client();
}

fn create_mongo_client() -> Client {
  let options = options::ClientOptions::builder()
    .hosts(vec![options::StreamAddress {
      hostname: "localhost".into(),
      port: Some(27017),
    }])
    .build();

  Client::with_options(options).expect("Failed to initialize standalone client.")
}

// fn collection(coll_name: &str) -> Collection {
//   MONGO.db("myblog").collection(coll_name)
// }
