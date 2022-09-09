use crate::article::article_statistics::ArticleStatisticsHttp;
use crate::tag::tag_base::TagHttp;
use crate::user::simple_user::SimpleUserHttp;

pub mod article_base;
pub mod article_statistics;
pub mod article_complete;
pub mod article_status;

pub trait Article {
    fn get_id(&self) -> i64;

    fn get_user_id(&self) -> i64;

    fn set_tag_list(&mut self, tag_list: Vec<TagHttp>);

    fn set_statistics(&mut self, statistics: ArticleStatisticsHttp);

    fn set_user(&mut self, user: SimpleUserHttp);

    fn set_content(&mut self, content: String);
}
