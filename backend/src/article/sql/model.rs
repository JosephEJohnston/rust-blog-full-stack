use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, QueryableByName, table};
use serde::{Deserialize, Serialize};
use share::article::ArticleHttp;

table! {
    article (id) {
        id -> Nullable<Bigint>,
        user_id -> Bigint,
        title -> Varchar,
        content -> Nullable<Text>,
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

    pub content: Option<String>,

    pub outline: String,

    pub status: i8,

    pub create_time: Option<NaiveDateTime>,

    pub modify_time: Option<NaiveDateTime>,
}

impl From<ArticleHttp> for ArticleDB {
    fn from(article: ArticleHttp) -> Self {
        ArticleDB {
            id: article.id,
            user_id: article.user_id,
            title: article.title,
            content: article.content,
            outline: article.outline,
            status: 0,
            create_time: None,
            modify_time: None
        }
    }
}

impl Into<ArticleHttp> for ArticleDB {
    fn into(self) -> ArticleHttp {
        ArticleHttp {
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            content: self.content,
            outline: self.outline,
            create_time: self.create_time.unwrap().timestamp(),
        }
    }
}

