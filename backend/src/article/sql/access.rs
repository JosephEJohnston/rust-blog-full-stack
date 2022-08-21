#![allow(dead_code)]

use diesel::{QueryDsl, QueryResult, RunQueryDsl};
use diesel::sql_types::Bigint;
use diesel::prelude::*;
use crate::article::sql::model::ArticleDB;
use crate::sql_conn::get_connection;

pub fn list_article_sql(user_id: i64) -> QueryResult<Vec<ArticleDB>> {
    let conn = &mut get_connection();

    diesel::sql_query("SELECT * FROM article WHERE user_id = ? AND status = 0 ORDER BY id DESC")
        .bind::<Bigint, _>(user_id)
        .get_results(conn)
}

pub fn test_sql() -> QueryResult<Vec<ArticleDB>> {
    use crate::article::sql::model::article::dsl::*;

    let conn = &mut get_connection();

    article
        .filter(user_id.eq(1))
        .select((
            id,
            user_id,
            content,
            outline,
            status,
            create_time,
            modify_time,
        ))
        .load::<ArticleDB>(conn)
}

#[cfg(test)]
mod tests {
    use crate::article::sql::access::test_sql;

    #[test]
    fn test() {
        if let Ok(res) = test_sql() {
            println!("{:?}", res);
        }
    }
}
