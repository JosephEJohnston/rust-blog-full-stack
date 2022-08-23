use rocket::fairing::AdHoc;
use rocket::{FromForm, get, routes};
use rocket::serde::json::Json;
use share::article::ArticleHttp;
use crate::article::sql::access::list_article_sql;
use crate::article::sql::model::ArticleDB;
use crate::tag::sql::access::list_tag_sql;
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
fn list_article_http(opt: ListArticleOptions) -> Json<Vec<ArticleHttp>> {
    let opt = list_article_sql(opt.user_id);

    if opt.is_none() {
        return Json(Vec::new());
    }

    let mut articles = opt.unwrap().into_iter()
        .map(|db: ArticleDB| <ArticleDB as Into<ArticleHttp>>::into(db))
        .collect();

    Json(articles)
}

fn set_with_tag_list(articles: &mut Vec<ArticleHttp>) {
    let entity_id_list = articles.iter()
        .map(|article| article.id.unwrap())
        .collect();

    let tag_relation_list =
        if let Some(tag_relation_list) = list_tag_relation_sql(
            entity_id_list, 1) {

            tag_relation_list
        } else {
            Vec::new()
        };

    let tag_id_list = tag_relation_list
        .iter()
        .map(|r| r.tag_id)
        .collect();

    let tag_list =
        if let Some(tag_list) = list_tag_sql(tag_id_list) {
            tag_list
        } else {
            Vec::new()
        };


    // tag_list.iter()
    //     .group_by(|tag| tag.)
}

