use std::error::Error;
use gloo::net::http::Request;
use share::article::article_base::ArticleListItemHttp;

pub async fn list_article_from_http() -> Result<Vec<ArticleListItemHttp>, Box<dyn Error>> {
    let url = "http://localhost:8000/article/list?user_id=1";

    let articles: Vec<ArticleListItemHttp> =
        Request::get(url)
            .send().await?
            .json().await?;

    Ok(articles)
}