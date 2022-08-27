use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, QueryableByName, table};
use serde::{Deserialize, Serialize};
use share::article::article_base::ArticleListItemHttp;
use share::article::article_complete::ArticleCompleteHttp;

table! {
    article (id) {
        id -> Nullable<Bigint>,
        user_id -> Bigint,
        title -> Varchar,
        outline -> Varchar,
        status -> Tinyint,
        create_time -> Nullable<Datetime>,
        modify_time -> Nullable<Datetime>,
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, QueryableByName)]
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
            status: 0,
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
            create_time: self.create_time,
        }
    }
}

impl Into<ArticleCompleteHttp> for ArticleDB {
    fn into(self) -> ArticleCompleteHttp {
        ArticleCompleteHttp {
            id: self.id,
            user_id: self.user_id,
            title: self.title.clone(),
            outline: self.outline.clone(),
            content: None,
            tag_list: None
        }
    }
}