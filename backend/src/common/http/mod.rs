use rocket::fairing::AdHoc;

mod upload;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Common", |rocket| async {
        rocket.attach(upload::stage())
    })
}
