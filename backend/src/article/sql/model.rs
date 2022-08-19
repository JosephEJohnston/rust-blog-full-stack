use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, QueryableByName, table};
use serde::{Deserialize, Serialize};

table! {
    article (id) {
        id -> Bigint,
        user_id -> Bigint,
        content -> Text,
        outline -> Varchar,
        status -> Tinyint,
        create_time -> Datetime,
        modify_time -> Datetime,
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, QueryableByName)]
#[diesel(table_name = article)]
pub struct ArticleDB {
    pub id: i64,

    pub user_id: i64,

    pub content: String,

    pub outline: String,

    pub status: i8,

    pub create_time: NaiveDateTime,

    pub modify_time: NaiveDateTime,
}

