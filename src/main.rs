#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::response::content;
use rocket_contrib::json::Json;
use serde::Serialize;

mod models;

#[derive(Serialize)]
struct Task {
    title: String,
    active: bool,
}

mod other {
    #[get("/world")]
    pub fn world() -> &'static str {
        "Hello, world!"
    }
}

#[get("/hello")]
pub fn hello() -> &'static str {
    models::db::init_db().await();
    models::db::test()
}

#[get("/json")]
pub fn show_json() -> content::Json<&'static str> {
    content::Json("{ 'hi': 'world!' }")
}

#[get("/task")]
fn task() -> Json<Task> {
    Json(Task {
        title: "测试任务aaaa".to_string(),
        active: true,
    })
}

#[get("/config")]
fn get_config() -> &'static str {
    let address = "aaaa";
    address
}

fn main() {
    rocket::ignite()
        .mount("/", routes![hello, other::world, show_json, task])
        .launch();
}
