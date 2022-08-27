use serde::{Deserialize, Serialize};
use crate::article::Article;
use crate::article::article_statistics::ArticleStatisticsHttp;
use crate::tag::tag_base::TagHttp;
use crate::user::simple_user::SimpleUserHttp;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ArticleCompleteHttp {
    pub id: Option<i64>,

    pub user_id: i64,

    pub title: String,

    pub outline: String,

    pub content: Option<String>,

    pub tag_list: Option<Vec<TagHttp>>,
}

impl Default for ArticleCompleteHttp {
    fn default() -> Self {
        ArticleCompleteHttp {
            id: None,
            user_id: 0,
            title: "".to_string(),
            outline: "".to_string(),
            content: None,
            tag_list: None
        }
    }
}

impl Article for ArticleCompleteHttp {

    fn get_id(&self) -> i64 {
        self.id.unwrap()
    }

    fn get_user_id(&self) -> i64 {
        self.user_id
    }

    fn set_tag_list(&mut self, tag_list: Vec<TagHttp>) {
        self.tag_list = Some(tag_list);
    }

    fn set_statistics(&mut self, _statistics: ArticleStatisticsHttp) {

    }

    fn set_user(&mut self, _user: SimpleUserHttp) {

    }

    fn set_content(&mut self, content: String) {
        self.content = Some(content);
    }
}