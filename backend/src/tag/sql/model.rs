use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, QueryableByName, table};
use serde::{Deserialize, Serialize};

table! {
    tag (id) {
        id -> Bigint,
        user_id -> Bigint,
        name -> Varchar,
        status -> Tinyint,
        create_time -> Datetime,
        modify_time -> Datetime,
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, QueryableByName)]
#[diesel(table_name = tag)]
pub struct TagDB {
    pub id: i64,

    pub user_id: i64,

    pub name: String,

    pub status: i8,

    pub create_time: NaiveDateTime,

    pub modify_time: NaiveDateTime,
}

