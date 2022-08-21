#![allow(dead_code)]

use diesel::{QueryResult, RunQueryDsl};
use diesel::sql_types::Bigint;
use crate::article::sql::model::ArticleDB;
use crate::sql_conn::get_connection;

pub fn list_article_sql(user_id: i64) -> QueryResult<Vec<ArticleDB>> {
    let conn = &mut get_connection();

    diesel::sql_query("SELECT * FROM article WHERE user_id = ? AND status = 0 ORDER BY id DESC")
        .bind::<Bigint, _>(user_id)
        .get_results(conn)
}

