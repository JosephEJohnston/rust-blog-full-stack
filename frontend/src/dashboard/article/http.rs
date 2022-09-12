use std::error::Error;
use gloo::net::http::Request;
use share::article::article_complete::ArticleCompleteHttp;
use share::article::article_status::ArticleStatusHttp;

// todo http 接口 result
pub async fn add_article_http(article: &ArticleCompleteHttp) -> Result<i64, Box<dyn Error>> {
    let url = "/api/article/add";

    let id: i64 = Request::post(url)
        .json(article)?
        .send().await?
        .json().await?;

    Ok(id)
}

pub async fn update_article_http(article: &ArticleCompleteHttp) -> Result<i64, Box<dyn Error>> {
    let url = "/api/article/update";

    let id: i64 = Request::post(url)
        .json(article)?
        .send().await?
        .json().await?;

    Ok(id)
}

pub async fn update_article_status(article_status: ArticleStatusHttp) -> Result<i64, Box<dyn Error>> {
    let url = "/api/article/update/status";

    let id: i64 = Request::post(url)
        .json(&article_status)?
        .send().await?
        .json().await?;

    Ok(id)
}
