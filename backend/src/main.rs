mod cors;
mod user;
mod tag;
mod article;
mod utils;

use rocket::{get, launch, options, routes};
use crate::cors::CORS;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .attach(article::http::stage())
        .mount("/api", routes![index, all_options])
}
