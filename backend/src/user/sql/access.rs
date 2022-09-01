#![allow(dead_code)]

use diesel::{ExpressionMethods, QueryResult, RunQueryDsl};
use diesel::sql_types::Varchar;
use diesel::prelude::*;
use crate::utils::sql::sql_conn::get_connection;
use crate::user::sql::model::user::dsl::*;
use crate::user::sql::model::UserDB;


fn get(email_: String) -> QueryResult<UserDB> {
    let conn = &mut get_connection();

    diesel::sql_query("SELECT * FROM user WHERE email = ? AND status = 0 LIMIT 1")
        .bind::<Varchar, _>(email_.clone())
        .get_result(conn)
}

pub fn list_user(user_id_list: Vec<i64>) -> Option<Vec<UserDB>> {
    let conn = &mut get_connection();

    let query_result = user
        .filter(id.eq_any(user_id_list))
        .load::<UserDB>(conn);

    return if let Ok(res) = query_result {
        Some(res)
    } else {
        None
    };
}

