#![allow(dead_code)]

use diesel::{QueryDsl, RunQueryDsl};
use diesel::prelude::*;
use share::article::http::ListArticleOptions;
use crate::article::sql::model::article::dsl::*;
use crate::article::sql::model::{ArticleDB, ListArticleOptionsSql};
use crate::utils::sql::sql_conn::get_connection;

pub fn list_article_sql(opts: ListArticleOptions) -> Option<Vec<ArticleDB>> {
    let opts = ListArticleOptionsSql::from(opts);

    let conn = &mut get_connection();

    let query = article
        .filter(user_id.eq(opts.user_id))
        .into_boxed();

    let query = query
        // todo 明天再说
        .filter(status.eq(opts.status.status.unwrap()))
        .limit(opts.page.limit)
        .offset(opts.page.offset)
        .order(create_time.desc())
        .load::<ArticleDB>(conn);

    return if let Ok(res) = query {
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