use std::error::Error;
use chrono::format::format;
use gloo::console::log;
use gloo::net::http::Request;
use wasm_bindgen::JsValue;
use share::article::article_complete::ArticleCompleteHttp;

// todo http 接口 result
pub async fn add_article_http(article: ArticleCompleteHttp) -> Result<i64, Box<dyn Error>> {
    let url = "http://localhost:8000/article/add";

    log!(format!("{:?}", JsValue::from_serde(&article).unwrap()));

    // todo 继续修错
    let id: i64 = Request::post(url)
        .body(JsValue::from_serde(&article).unwrap().as_string().unwrap())
        .send().await?
        .json().await?;

    Ok(id)
}


