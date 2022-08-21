use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ArticleHttp {
    pub id: Option<i64>,

    pub user_id: i64,

    pub content: Option<String>,

    pub outline: String,

    pub create_time: i64,
}

