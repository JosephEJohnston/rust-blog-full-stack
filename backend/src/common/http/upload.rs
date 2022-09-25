use ubyte::ToByteUnit;
use rocket::fairing::AdHoc;
use rocket::{Data, post, routes};
use rocket::serde::json::Json;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Common", |rocket| async {
        rocket.mount("/api/common", routes![upload_file])
    })
}

// todo 彻底改用 result
#[post("/upload", data="<file>")]
async fn upload_file(file: Data<'_>) -> Json<String> {
    file.open(2048_i32.kibibytes()).into_file("/test").await.unwrap();
    Json("/test".to_string())
}
