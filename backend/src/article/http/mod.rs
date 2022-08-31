use rocket::fairing::AdHoc;

pub mod access;
pub mod modify;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Article", |rocket| async {
        rocket.attach(access::stage())
            .attach(modify::stage())
    })
}