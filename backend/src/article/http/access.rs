use rocket::get;
use rocket::serde::json::Json;
use share::article::ArticleHttp;
use crate::article::sql::access::list_article;
use crate::article::sql::model::ArticleDB;

#[get("/")]
fn get_articles() -> Json<Vec<ArticleHttp>> {
    let articles = list_article(1).unwrap();

    let articles = articles.into_iter()
        .map(|db: ArticleDB| <ArticleDB as Into<ArticleHttp>>::into(db))
        .collect();

    Json(articles)
}