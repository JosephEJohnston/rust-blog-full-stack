#![allow(dead_code)]

use diesel::{QueryDsl, RunQueryDsl};
use diesel::prelude::*;
use crate::article::article_content::sql::model::article_content::dsl::*;
use crate::article::article_content::sql::model::ArticleContentDB;
use crate::utils::sql::sql_conn::get_connection;

pub fn list_article_content(article_id_list: Vec<i64>) -> Option<Vec<ArticleContentDB>> {
    let conn = &mut get_connection();

    let query_result = article_content
        .filter(article_id.eq_any(article_id_list))
        .load::<ArticleContentDB>(conn);

    return if let Ok(res) = query_result {
        Some(res)
    } else {
        None
    };
}

#[test]
fn test() {
    println!("{:?}", list_article_content(vec![1]));
}

