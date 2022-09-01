use share::article::article_complete::ArticleCompleteHttp;
use crate::article::sql::model::ArticleDB;
use crate::article::sql::modify::insert;

pub struct InsertArticleService {
    article: ArticleCompleteHttp,
    id: Option<i64>,
}

impl InsertArticleService {
    pub fn new(article: ArticleCompleteHttp) -> InsertArticleService {
        InsertArticleService {
            article,
            id: None,
        }
    }

    pub fn insert_article_base(&mut self) {
        let result = insert(ArticleDB::from(
            self.article.clone()));

        if let Ok(article_id) = result {
            self.id = Some(article_id);
        }
    }
}