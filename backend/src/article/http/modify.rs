use rocket::fairing::AdHoc;
use rocket::{post, routes};
use rocket::serde::json::Json;
use share::article::article_complete::ArticleCompleteHttp;
use crate::article::service::modify::InsertArticleService;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Article", |rocket| async {
        rocket.mount("/article", routes![add_article])
    })
}

#[post("/add", data = "<article>")]
fn add_article(article: Json<ArticleCompleteHttp>) -> Json<()> {
    let article = article.into_inner();

    let mut service = InsertArticleService::new(article);

    service.insert_article_base();

    Json(())
}

