use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, QueryableByName, table};
use serde::{Deserialize, Serialize};
use share::user::simple_user::SimpleUserHttp;

table! {
    user (id) {
        id -> Nullable<Bigint>,
        name -> Varchar,
        email -> Varchar,
        phone -> Varchar,
        password -> Varchar,
        encrypted_password -> Varchar,
        status -> Tinyint,
        create_time -> Nullable<Datetime>,
        modify_time -> Nullable<Datetime>,
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, QueryableByName)]
#[diesel(table_name = user)]
pub struct UserDB {
    pub id: Option<i64>,

    pub name: String,

    pub email: String,

    pub phone: String,

    pub password: String,

    pub encrypted_password: String,

    pub status: i8,

    pub create_time: Option<NaiveDateTime>,

    pub modify_time: Option<NaiveDateTime>,
}

impl Into<SimpleUserHttp> for UserDB {
    fn into(self) -> SimpleUserHttp {
        SimpleUserHttp {
            id: self.id.unwrap(),
            name: self.name.clone(),
        }
    }
}
