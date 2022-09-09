#![allow(dead_code)]

use chrono::{Local, NaiveDateTime};
use diesel::{QueryDsl, RunQueryDsl};
use diesel::{select};
use diesel::prelude::*;
use diesel::result::Error;
use crate::article::sql::model::ArticleDB;
use crate::utils::sql::sql_conn::get_connection;
use crate::article::sql::model::article::dsl::*;
use crate::utils::sql::function::last_insert_id;

pub fn insert(article_: ArticleDB) -> QueryResult<i64> {
    let conn = &mut get_connection();

    let insert_id = conn.transaction(|conn| {
        let result = diesel::insert_into(article)
            .values(vec![article_])
            .execute(conn);

        if let Ok(_n) = result {
            Ok(select(last_insert_id()).get_result::<i64>(conn))
        } else {
            Err(Error::RollbackTransaction)
        }
    })?;

    insert_id
}

pub fn update_sql(article_: ArticleDB) -> QueryResult<usize> {
    let conn = &mut get_connection();

    let result = diesel::update(article
        .filter(id.eq(article_.id.unwrap())))
        .set((
            title.eq(article_.title),
            outline.eq(article_.outline),
            modify_time.eq(NaiveDateTime::from_timestamp_opt(
                Local::now().timestamp(), 0)),
        ))
        .execute(conn);

    result
}

pub fn update_status_sql(article_id: i64, status_: i8) -> QueryResult<usize> {
    let conn = &mut get_connection();

    diesel::update(article
        .filter(id.eq(article_id)))
        .set((
            status.eq(status_),
            modify_time.eq(NaiveDateTime::from_timestamp_opt(
                Local::now().timestamp(), 0)),
        ))
        .execute(conn)
}

#[cfg(test)]
mod tests {
    use crate::article::sql::model::ArticleDB;
    use crate::article::sql::modify::insert;

    #[test]
    fn test_insert() {
        let article = ArticleDB {
            id: None,
            user_id: 1,
            title: "测试文章插入4".to_string(),
            outline: "测试文章插入4".to_string(),
            status: 0,
            create_time: None,
            modify_time: None
        };

        if let Ok(article_id) = insert(article) {
            println!("Insert article id: {:?}", article_id);
        }
    }
}