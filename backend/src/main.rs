mod cors;

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
