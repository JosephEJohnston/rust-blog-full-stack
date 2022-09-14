use rocket::fairing::AdHoc;
use rocket::{FromForm, get, post, routes};
use rocket::serde::json::Json;
use share::article::article_base::ArticleListItemHttp;
use share::article::article_complete::ArticleCompleteHttp;
use share::article::http::ListArticleOptions;
use share::utils::page::Pagination;
use crate::article::article_content::service::access::ArticleContentAccessService;
use crate::article::http::enums::MarkOpt;
use crate::article::service::base::{ArticleService, ArticleSingleService};
use crate::article::sql::access::{get_article_sql, list_article_sql};
use crate::article::sql::model::ArticleDB;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Article", |rocket| async {
        rocket.mount("/api/article", routes![list_article, get_article])
    })
}


#[post("/list", data = "<opt>")]
fn list_article(opt: Json<ListArticleOptions>) -> Json<Pagination<Vec<ArticleListItemHttp>>> {
    let opt = opt.into_inner();

    let res = list_article_sql(opt.clone());

    let article_list: Vec<ArticleListItemHttp> = res.unwrap().into_iter()
        .map(|db: ArticleDB| <ArticleDB as Into<ArticleListItemHttp>>::into(db))
        .collect();

    let mut article_service = ArticleService::new(article_list);

    article_service.each_set_with_tag_list();
    article_service.each_set_with_statistics();
    article_service.each_set_with_user();

    Json(article_service.to_page(opt))
}

#[derive(FromForm)]
struct GetArticleOptions {
    pub id: i64,
    pub markdown_opt: i8,
}

#[get("/get?<opt..>")]
fn get_article(opt: GetArticleOptions) -> Json<ArticleCompleteHttp> {
    if let Some(article) = get_article_sql(opt.id) {
        let article: ArticleCompleteHttp = article.into();

        let mut service = ArticleSingleService::new(article);
        service.set_tag_list()
            .set_content();

        let mut content_service =
            ArticleContentAccessService::new(service.consume());
        content_service.render_markdown(MarkOpt(opt.markdown_opt));

        Json(content_service.done())
    } else {
        Json(ArticleCompleteHttp::default())
    }
}