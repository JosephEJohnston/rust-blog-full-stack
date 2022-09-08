use rocket::fairing::AdHoc;
use rocket::{FromForm, get, routes};
use rocket::serde::json::Json;
use share::article::article_base::ArticleListItemHttp;
use share::article::article_complete::ArticleCompleteHttp;
use crate::article::service::base::{ArticleService, ArticleSingleService};
use crate::article::sql::access::{get_article_sql, list_article_sql};
use crate::article::sql::model::ArticleDB;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Article", |rocket| async {
        rocket.mount("/api/article", routes![list_article, get_article])
    })
}

#[derive(FromForm)]
struct ListArticleOptions {
    pub user_id: i64,
}

#[get("/list?<opt..>")]
fn list_article(opt: ListArticleOptions) -> Json<Vec<ArticleListItemHttp>> {
    let opt = list_article_sql(opt.user_id);

    if opt.is_none() {
        return Json(Vec::new());
    }

    let article_list: Vec<ArticleListItemHttp> = opt.unwrap().into_iter()
        .map(|db: ArticleDB| <ArticleDB as Into<ArticleListItemHttp>>::into(db))
        .collect();

    let mut article_service = ArticleService::new(article_list);

    article_service.each_set_with_tag_list();
    article_service.each_set_with_statistics();
    article_service.each_set_with_user();

    Json(article_service.consume())
}

#[derive(FromForm)]
struct GetArticleOptions {
    pub id: i64,
}

#[get("/get?<opt..>")]
fn get_article(opt: GetArticleOptions) -> Json<ArticleCompleteHttp> {
    if let Some(article) = get_article_sql(opt.id) {
        let article: ArticleCompleteHttp = article.into();

        let mut service = ArticleSingleService::new(article);

        service.set_tag_list();
        service.set_content();

        Json(service.consume())
    } else {
        Json(ArticleCompleteHttp::default())
    }
}