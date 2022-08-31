use gloo::net::http::Request;
use wasm_bindgen::JsValue;
use share::article::article_complete::ArticleCompleteHttp;

// todo http 接口 result
pub async fn add_article_http(article: ArticleCompleteHttp) {
    let url = "http://localhost:8000/article/add";

    Request::post(url)
        .body(JsValue::from_serde(&article))
        .send().await?
        .json().await?;
}

