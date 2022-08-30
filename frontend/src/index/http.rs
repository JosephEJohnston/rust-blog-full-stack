use std::error::Error;
use gloo::net::http::Request;
use share::article::article_base::ArticleListItemHttp;
use share::article::article_complete::ArticleCompleteHttp;

// todo 全局文章接口和用户文章接口
pub async fn list_article_from_http() -> Result<Vec<ArticleListItemHttp>, Box<dyn Error>> {
    let url = "http://localhost:8000/article/list?user_id=1";

    let articles: Vec<ArticleListItemHttp> =
        Request::get(url)
            .send().await?
            .json().await?;

    Ok(articles)
}

pub async fn get_article_from_http(article_id: i64) -> Result<ArticleCompleteHttp, Box<dyn Error>> {
    let mut url = "http://localhost:8000/article/get?id=".to_string();
    url.push_str(article_id.to_string().as_str());

    let article: ArticleCompleteHttp =
        Request::get(url.as_str())
            .send().await?
            .json().await?;

    Ok(article)
}