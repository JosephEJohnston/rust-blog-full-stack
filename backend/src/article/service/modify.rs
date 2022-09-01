use share::article::article_complete::ArticleCompleteHttp;
use crate::article::article_content::sql::model::ArticleContentDB;
use crate::article::article_content::sql::modify::insert_article_content;
use crate::article::article_statistics::sql::model::ArticleStatisticsDB;
use crate::article::article_statistics::sql::modify::insert_article_statistics;
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

    pub fn insert_article_base(&mut self) -> &Self {
        let result = insert(ArticleDB::from(
            self.article.clone()));

        if let Ok(article_id) = result {
            self.id = Some(article_id);
        }

        self
    }

    pub fn insert_article_content(&self) -> &Self {
        if let Some(article_id) = self.id {
            if let Ok(_n) = insert_article_content(ArticleContentDB {
                id: None,
                article_id,
                content: self.article.content.as_ref()
                    .unwrap_or(&"".to_string()).clone(),
                status: 0,
                create_time: None,
                modify_time: None,
            }) {}
        }

        self
    }

    pub fn insert_article_statistics(&self) -> &Self {
        if let Some(article_id) = self.id {
            if let Ok(_n) = insert_article_statistics(ArticleStatisticsDB {
                id: None,
                article_id,
                read_count: 0,
                status: 0,
                create_time: None,
                modify_time: None,
            }) {}
        }

        self
    }

    pub fn done(self) -> i64 {
        if let Some(id) = self.id {
            id
        } else {
            -1
        }
    }
}