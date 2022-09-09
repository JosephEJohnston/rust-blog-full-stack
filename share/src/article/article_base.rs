use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::article::Article;
use crate::article::article_statistics::ArticleStatisticsHttp;
use crate::tag::tag_base::TagHttp;
use crate::user::simple_user::SimpleUserHttp;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ArticleListItemHttp {
    pub id: Option<i64>,

    pub user_id: i64,

    pub title: String,

    pub outline: String,

    pub user: Option<SimpleUserHttp>,

    pub statistics: Option<ArticleStatisticsHttp>,

    pub tag_list: Option<Vec<TagHttp>>,

    pub status: i8,

    pub create_time: Option<NaiveDateTime>,
}

impl Article for ArticleListItemHttp {

    fn get_id(&self) -> i64 {
        self.id.unwrap()
    }

    fn get_user_id(&self) -> i64 {
        self.user_id
    }

    fn set_tag_list(&mut self, tag_list: Vec<TagHttp>) {
        self.tag_list = Some(tag_list);
    }

    fn set_statistics(&mut self, statistics: ArticleStatisticsHttp) {
        self.statistics = Some(statistics);
    }

    fn set_user(&mut self, user: SimpleUserHttp) {
        self.user = Some(user);
    }

    fn set_content(&mut self, _content: String) {
    }
}