use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::article::article_statistics::ArticleStatisticsHttp;
use crate::tag::tag_base::TagHttp;
use crate::user::simple_user::SimpleUserHttp;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ArticleHttp {
    pub id: Option<i64>,

    pub user_id: i64,

    pub title: String,

    pub outline: String,

    pub user: Option<SimpleUserHttp>,

    pub content: Option<String>,

    pub statistics: Option<ArticleStatisticsHttp>,

    pub tag_list: Option<Vec<TagHttp>>,

    pub create_time: Option<NaiveDateTime>,
}

