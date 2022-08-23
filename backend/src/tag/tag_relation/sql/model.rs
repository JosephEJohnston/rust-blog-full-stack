use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, QueryableByName, table};
use serde::{Deserialize, Serialize};

table! {
    tag_relation (id) {
        id -> Nullable<Bigint>,
        tag_id -> Bigint,
        entity_id -> Bigint,
        entity_type -> Integer,
        status -> Tinyint,
        create_time -> Nullable<Datetime>,
        modify_time -> Nullable<Datetime>,
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, QueryableByName)]
#[diesel(table_name = tag_relation)]
pub struct TagRelationDB {
    pub id: Option<i64>,

    pub tag_id: i64,

    pub entity_id: i64,

    pub entity_type: i32,

    pub status: i8,

    pub create_time: Option<NaiveDateTime>,

    pub modify_time: Option<NaiveDateTime>,
}


