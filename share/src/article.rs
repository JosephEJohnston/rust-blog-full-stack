use serde::{Deserialize, Serialize};
use crate::tag::TagHttp;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ArticleHttp {
    pub id: Option<i64>,

    pub user_id: i64,

    pub title: String,

    pub content: Option<String>,

    pub outline: String,

    pub tag_list: Option<Vec<TagHttp>>,

    pub create_time: i64,
}

