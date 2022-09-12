use share::article::article_complete::ArticleCompleteHttp;
use share::article::article_status::ArticleStatusHttp;
use crate::article::article_content::sql::model::ArticleContentDB;
use crate::article::article_content::sql::modify::{insert_article_content, update_article_content_sql};
use crate::article::article_statistics::sql::model::ArticleStatisticsDB;
use crate::article::article_statistics::sql::modify::insert_article_statistics;
use crate::article::sql::model::ArticleDB;
use crate::article::sql::modify::{insert, update_sql, update_status_sql};

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
            &self.article));

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
                status: 1,
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
                status: 1,
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

pub struct UpdateArticleService {
    article: ArticleCompleteHttp,
}

impl UpdateArticleService {
    pub fn new(article: ArticleCompleteHttp) -> UpdateArticleService {
        UpdateArticleService {
            article
        }
    }

    pub fn update_base(&self) -> &Self {
        // todo 未来再说
        match update_sql(ArticleDB::from(&self.article)) {
            Ok(_id) => (),
            Err(_e) => (),
        }

        self
    }

    pub fn update_content(&self) -> &Self {
        match update_article_content_sql(ArticleContentDB::from(&self.article)) {
            Ok(_id) => (),
            Err(_e) => (),
        }

        self
    }

    pub fn update_tag_list(&self) -> &Self {

        self
    }

    pub fn done(self) -> i64 {
        self.article.id.unwrap()
    }
}

pub struct UpdateArticleStatusService {
    article_status: ArticleStatusHttp,
}

impl UpdateArticleStatusService {
    pub fn new(article_status: ArticleStatusHttp) -> UpdateArticleStatusService {
        UpdateArticleStatusService {
            article_status
        }
    }

    pub fn update_status(&self) -> &Self {
        match update_status_sql(
            self.article_status.id.clone(),
            self.article_status.status
        ) {
            Ok(_size) => (),
            Err(_e) => (),
        }

        self
    }

    pub fn done(self) -> i64 {
        self.article_status.id
    }
}