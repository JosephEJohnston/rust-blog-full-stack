use diesel::{QueryResult, RunQueryDsl};
use diesel::sql_types::Bigint;
use crate::article::sql::model::ArticleDB;
use crate::sql_conn::get_connection;

fn get() -> QueryResult<Vec<ArticleDB>> {
    let user_id: i64 = 1;

    let conn = &mut get_connection();

    diesel::sql_query("")
        .bind::<Bigint, _>(user_id)
        .get_results(conn)
}