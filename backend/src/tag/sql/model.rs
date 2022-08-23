use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, QueryableByName, table};
use serde::{Deserialize, Serialize};
use share::tag::TagHttp;

table! {
    tag (id) {
        id -> Nullable<Bigint>,
        user_id -> Bigint,
        name -> Varchar,
        status -> Tinyint,
        create_time -> Nullable<Datetime>,
        modify_time -> Nullable<Datetime>,
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, QueryableByName)]
#[diesel(table_name = tag)]
pub struct TagDB {
    pub id: Option<i64>,

    pub user_id: i64,

    pub name: String,

    pub status: i8,

    pub create_time: Option<NaiveDateTime>,

    pub modify_time: Option<NaiveDateTime>,
}

impl From<TagHttp> for TagDB {
    fn from(tag: TagHttp) -> Self {
        TagDB {
            id: None,
            user_id: tag.user_id,
            name: tag.name.clone(),
            status: 0,
            create_time: None,
            modify_time: None
        }
    }
}

impl Into<TagHttp> for TagDB {
    fn into(self) -> TagHttp {
        TagHttp {
            id: self.id.unwrap(),
            user_id: self.user_id,
            name: self.name.clone(),
        }
    }
}
