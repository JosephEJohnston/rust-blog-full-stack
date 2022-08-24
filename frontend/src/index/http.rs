use std::error::Error;
use gloo::net::http::Request;
use share::article::article_base::ArticleHttp;

pub async fn list_article_from_http() -> Result<Vec<ArticleHttp>, Box<dyn Error>> {
    let url = "http://localhost:8000/article/list?user_id=1";

    let articles: Vec<ArticleHttp> =
        Request::get(url)
            .send().await?
            .json().await?;

    Ok(articles)
}