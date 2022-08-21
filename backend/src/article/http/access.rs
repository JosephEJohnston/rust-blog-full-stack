use rocket::fairing::AdHoc;
use rocket::{get, routes};
use rocket::serde::json::Json;
use share::article::ArticleHttp;
use crate::article::sql::access::list_article_sql;
use crate::article::sql::model::ArticleDB;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Article", |rocket| async {
        rocket.mount("/articles", routes![list_article_http])
    })
}

#[get("/")]
fn list_article_http() -> Json<Vec<ArticleHttp>> {
    let articles = list_article_sql(1).unwrap();

    let articles = articles.into_iter()
        .map(|db: ArticleDB| <ArticleDB as Into<ArticleHttp>>::into(db))
        .collect();

    Json(articles)
}