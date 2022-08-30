use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, QueryableByName, table};
use serde::{Deserialize, Serialize};
use share::article::article_statistics::ArticleStatisticsHttp;

table! {
    article_statistics (id) {
        id -> Nullable<Bigint>,
        article_id -> Bigint,
        read_count -> Integer,
        status -> Tinyint,
        create_time -> Nullable<Datetime>,
        modify_time -> Nullable<Datetime>,
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, QueryableByName)]
#[diesel(table_name = article_statistics)]
pub struct ArticleStatisticsDB {
    pub id: Option<i64>,

    pub article_id: i64,

    pub read_count: i32,

    pub status: i8,

    pub create_time: Option<NaiveDateTime>,

    pub modify_time: Option<NaiveDateTime>,
}

impl Into<ArticleStatisticsHttp> for ArticleStatisticsDB {
    fn into(self) -> ArticleStatisticsHttp {
        ArticleStatisticsHttp {
            article_id: self.article_id,
            read_count: self.read_count,
        }
    }
}

