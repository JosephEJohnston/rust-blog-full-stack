#![allow(dead_code)]

use diesel::{QueryDsl, RunQueryDsl};
use diesel::prelude::*;
use crate::article::sql::model::article::dsl::*;
use crate::article::sql::model::ArticleDB;
use crate::sql_conn::get_connection;

pub fn list_article_sql(user_id_: i64) -> Option<Vec<ArticleDB>> {
    let conn = &mut get_connection();

    let query_result = article
        .filter(user_id.eq(user_id_))
        .order(create_time.desc())
        .load::<ArticleDB>(conn);

    return if let Ok(res) = query_result {
        Some(res)
    } else {
        None
    };
}

pub fn get_article_sql(id_: i64) -> Option<ArticleDB> {
    let conn = &mut get_connection();

    let query_result = article
        .filter(id.eq(id_))
        .limit(1)
        .load::<ArticleDB>(conn);

    return if let Ok(mut res) = query_result {
        if res.len() == 1 {
            Some(res.pop().unwrap())
        } else {
            None
        }
    } else {
        None
    };
}