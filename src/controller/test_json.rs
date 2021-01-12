use actix_web::{error, web, Error, HttpRequest, HttpResponse};
use futures::StreamExt;
use json::JsonValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MyObj {
  name: String,
  number: i32,
}

/// This handler uses json extractor
pub async fn index(item: web::Json<MyObj>) -> HttpResponse {
  println!("model: {:?}", &item);
  HttpResponse::Ok().json(item.0) // <- send response
}

/// This handler uses json extractor with limit
pub async fn extract_item(item: web::Json<MyObj>, req: HttpRequest) -> HttpResponse {
  println!("request: {:?}", req);
  println!("model: {:?}", item);

  HttpResponse::Ok().json(item.0) // <- send json response
}

const MAX_SIZE: usize = 262_144; // max payload size is 256k

/// This handler manually load request payload and parse json object
pub async fn index_manual(mut payload: web::Payload) -> Result<HttpResponse, Error> {
  // payload is a stream of Bytes objects
  let mut body = web::BytesMut::new();
  while let Some(chunk) = payload.next().await {
    let chunk = chunk?;
    // limit max size of in-memory payload
    if (body.len() + chunk.len()) > MAX_SIZE {
      return Err(error::ErrorBadRequest("overflow"));
    }
    body.extend_from_slice(&chunk);
  }

  // body is loaded, now we can deserialize serde-json
  let obj = serde_json::from_slice::<MyObj>(&body)?;
  Ok(HttpResponse::Ok().json(obj)) // <- send response
}

/// This handler manually load request payload and parse json-rust
pub async fn index_mjsonrust(body: web::Bytes) -> Result<HttpResponse, Error> {
  // body is loaded, now we can deserialize json-rust
  let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
  let injson: JsonValue = match result {
    Ok(v) => v,
    Err(e) => json::object! {"err" => e.to_string() },
  };
  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .body(injson.dump()),
  )
}
