use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, table};
use serde::{Deserialize, Serialize};
use share::article::article_base::ArticleListItemHttp;
use share::article::article_complete::ArticleCompleteHttp;
use share::article::http::ListArticleOptions;
use share::utils::status::StatusOptions;
use crate::utils::sql::page::PaginationSql;

table! {
    article (id) {
        id -> Nullable<Bigint>,
        user_id -> Bigint,
        title -> Varchar,
        outline -> Varchar,
        status -> TinyInt,
        create_time -> Nullable<Datetime>,
        modify_time -> Nullable<Datetime>,
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = article)]
pub struct ArticleDB {
    pub id: Option<i64>,

    pub user_id: i64,

    pub title: String,

    pub outline: String,

    pub status: i8,

    pub create_time: Option<NaiveDateTime>,

    pub modify_time: Option<NaiveDateTime>,
}

impl From<ArticleListItemHttp> for ArticleDB {
    fn from(article: ArticleListItemHttp) -> Self {
        ArticleDB {
            id: article.id,
            user_id: article.user_id,
            title: article.title,
            outline: article.outline,
            status: article.status,
            create_time: None,
            modify_time: None
        }
    }
}

impl Into<ArticleListItemHttp> for ArticleDB {
    fn into(self) -> ArticleListItemHttp {
        ArticleListItemHttp {
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            outline: self.outline,
            user: None,
            statistics: None,
            tag_list: None,
            status: self.status,
            create_time: self.create_time,
        }
    }
}

impl From<&ArticleCompleteHttp> for ArticleDB {
    fn from(article: &ArticleCompleteHttp) -> Self {
        ArticleDB {
            id: article.id.clone(),
            user_id: article.user_id,
            title: article.title.clone(),
            outline: article.outline.clone(),
            status: article.status,
            create_time: None,
            modify_time: None
        }
    }
}

impl Into<ArticleCompleteHttp> for ArticleDB {
    fn into(self) -> ArticleCompleteHttp {
        ArticleCompleteHttp {
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            outline: self.outline,
            content: None,
            tag_list: None,
            status: self.status,
        }
    }
}

pub struct ListArticleOptionsSql {
    pub user_id: i64,
    pub status: StatusOptions,
    pub page: PaginationSql,
}

impl From<ListArticleOptions> for ListArticleOptionsSql {
    fn from(opts: ListArticleOptions) -> Self {
        Self {
            user_id: opts.user_id,
            status: opts.status,
            page: PaginationSql::from(opts.page),
        }
    }
}


