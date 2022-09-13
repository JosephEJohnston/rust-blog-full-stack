use std::error::Error;
use gloo::net::http::Request;
use share::article::article_base::ArticleListItemHttp;
use share::article::article_complete::ArticleCompleteHttp;
use share::article::http::ListArticleOptions;

// todo 全局文章接口和用户文章接口
pub async fn list_article_http(opts: ListArticleOptions) -> Result<Vec<ArticleListItemHttp>, Box<dyn Error>> {
    let url = "/api/article/list";

    let articles: Vec<ArticleListItemHttp> =
        Request::post(url)
            .json(&opts)?
            .send().await?
            .json().await?;

    Ok(articles)
}

pub struct GetArticleOptions {
    pub id: i64,
    pub markdown_opt: i8,
}

pub async fn get_article_http(opts: GetArticleOptions) -> Result<ArticleCompleteHttp, Box<dyn Error>> {
    let article: ArticleCompleteHttp =
        Request::get("/api/article/get")
            .query([("id", opts.id.to_string())])
            .query([("markdown_opt", opts.markdown_opt.to_string())])
            .send().await?
            .json().await?;

    Ok(article)
}