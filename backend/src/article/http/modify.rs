use rocket::fairing::AdHoc;
use rocket::{post, routes};
use rocket::serde::json::Json;
use share::article::article_complete::ArticleCompleteHttp;
use share::article::article_status::ArticleStatus;
use crate::article::service::modify::{InsertArticleService, UpdateArticleService, UpdateArticleStatusService};

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Article", |rocket| async {
        rocket.mount("/api/article", routes![
            add_article,
            update_article,
            update_article_status
        ])
    })
}

#[post("/add", data = "<article>")]
fn add_article(article: Json<ArticleCompleteHttp>) -> Json<i64> {
    let article = article.into_inner();

    let mut service = InsertArticleService::new(article);

    service.insert_article_base()
        .insert_article_content()
        .insert_article_statistics();

    Json(service.done())
}

#[post("/update", data = "<article>")]
fn update_article(article: Json<ArticleCompleteHttp>) -> Json<i64> {
    let article = article.into_inner();

    let service = UpdateArticleService::new(article);

    service.update_base()
        .update_content()
        .update_tag_list();

    Json(service.done())
}

#[post("/update/status", data = "<article_status>")]
fn update_article_status(article_status: Json<ArticleStatus>) -> Json<i64> {
    let article_status = article_status.into_inner();

    let service = UpdateArticleStatusService::new(article_status);

    service.update_status();

    Json(service.done())
}