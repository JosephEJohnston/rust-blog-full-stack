use rocket::fairing::AdHoc;
use rocket::{post, routes};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use share::article::article_complete::ArticleCompleteHttp;
use crate::article::service::modify::InsertArticleService;
use crate::article::sql::model::ArticleDB;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Article", |rocket| async {
        rocket.mount("/article", routes![add_article])
    })
}

#[post("/add", data = "<article>")]
fn add_article(article: Json<ArticleCompleteHttp>) -> Json<()> {
    let article = article.into_inner();

    let service = InsertArticleService::new(article);


    Json(())
}

