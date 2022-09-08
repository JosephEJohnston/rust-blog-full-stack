use std::error::Error;
use gloo::net::http::Request;
use share::article::article_complete::ArticleCompleteHttp;

// todo http 接口 result
pub async fn add_article_http(article: &ArticleCompleteHttp) -> Result<i64, Box<dyn Error>> {
    let url = "/api/article/add";
    // let url = "http://localhost:8000/api/article/add";

    // todo 继续修错
    let id: i64 = Request::post(url)
        .json(article)?
        .send().await?
        .json().await?;

    Ok(id)
}


