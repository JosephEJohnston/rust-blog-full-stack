mod cors;
mod user;
mod tag;
mod article;
mod sql_conn;

use rocket::{get, launch, routes};
use crate::cors::CORS;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .mount("/", routes![index])
}
