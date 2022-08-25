use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ArticleStatisticsHttp {
    pub article_id: i64,

    pub read_count: i32,
}