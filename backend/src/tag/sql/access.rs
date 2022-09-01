#![allow(dead_code)]

use diesel::{QueryResult, RunQueryDsl};
use diesel::sql_types::Bigint;
use diesel::prelude::*;
use crate::utils::sql::sql_conn::get_connection;
use crate::tag::sql::model::tag::dsl::*;
use crate::tag::sql::model::TagDB;

pub fn get(user_id_: i64) -> QueryResult<Vec<TagDB>> {
    let conn = &mut get_connection();

    diesel::sql_query("SELECT * FROM tag WHERE user_id = ? AND status = 0")
        .bind::<Bigint, _>(user_id_)
        .get_results(conn)
}

pub fn list_tag_sql(tag_id_list: Vec<i64>) -> Option<Vec<TagDB>> {
    let conn = &mut get_connection();

    let query_result = tag
        .filter(id.eq_any(tag_id_list))
        .load::<TagDB>(conn);

    return if let Ok(res) = query_result {
        Some(res)
    } else {
        None
    };
}

#[test]
fn test() {
    println!("{:?}", list_tag_sql(vec![1, 2, 3]));
}

