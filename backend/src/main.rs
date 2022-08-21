mod cors;
mod user;
mod tag;
mod article;
mod sql_conn;

use rocket::{get, launch, routes};
use crate::article::http::access::stage;
use crate::cors::CORS;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .attach(stage())
        .mount("/", routes![index])
}
