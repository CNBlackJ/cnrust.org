use mongodb::{options::ClientOptions, Client};

pub async fn init_db() {
  // Parse a connection string into an options struct.
  let client_options = ClientOptions::parse("mongodb://localhost:27017")
    .await
    .unwrap();

  // Get a handle to the deployment.
  let client = Client::with_options(client_options).unwrap();

  // List the names of the databases in that deployment.
  for db_name in client.list_database_names(None, None).await.unwrap() {
    println!("{}", db_name);
  }
}

pub fn test() -> &'static str {
  "aa"
}
