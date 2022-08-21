#![allow(dead_code)]

use diesel::{QueryResult, RunQueryDsl};
use diesel::sql_types::Bigint;
use crate::sql_conn::get_connection;
use crate::tag::sql::model::TagDB;

fn get(user_id: i64) -> QueryResult<Vec<TagDB>> {
    let conn = &mut get_connection();

    diesel::sql_query("SELECT * FROM tag WHERE user_id = ? AND status = 0")
        .bind::<Bigint, _>(user_id)
        .get_results(conn)
}

