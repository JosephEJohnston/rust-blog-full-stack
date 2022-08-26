use std::collections::HashMap;
use rocket::fairing::AdHoc;
use rocket::{FromForm, get, routes};
use rocket::serde::json::Json;
use share::article::article_base::ArticleListItemHttp;
use share::tag::tag_base::TagHttp;
use crate::article::service::base::ArticleService;
use crate::article::sql::access::list_article_sql;
use crate::article::sql::model::ArticleDB;
use crate::tag::sql::access::list_tag_sql;
use crate::tag::sql::model::TagDB;
use crate::tag::tag_relation::sql::access::list_tag_relation_sql;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Article", |rocket| async {
        rocket.mount("/article", routes![list_article_http])
    })
}

#[derive(FromForm)]
struct ListArticleOptions {
    pub user_id: i64,
}

#[get("/list?<opt..>")]
fn list_article_http(opt: ListArticleOptions) -> Json<Vec<ArticleListItemHttp>> {
    let opt = list_article_sql(opt.user_id);

    if opt.is_none() {
        return Json(Vec::new());
    }

    let mut article_service = ArticleService::new(opt.unwrap());

    article_service.each_set_with_tag_list();
    article_service.each_set_with_statistics();
    article_service.each_set_with_user();

    Json(article_service.consume())
}
