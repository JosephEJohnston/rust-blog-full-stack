use rocket::fairing::AdHoc;
use rocket::{post, routes};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Article", |rocket| async {
        rocket.mount("/article", routes![add_article])
    })
}

#[derive(Debug, Serialize, Deserialize)]
struct TestJson {
    pub test: i64,
}

#[post("/add", data = "<data>")]
fn add_article(data: Json<TestJson>) {
    let data = data.into_inner();
    println!("{:?}", data.test);
}

