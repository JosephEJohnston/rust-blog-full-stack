mod cors;
mod user;
mod tag;
mod article;
mod utils;

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
        .attach(article::http::stage())
        .mount("/", routes![index])
}
