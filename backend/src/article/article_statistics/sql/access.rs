use diesel::{QueryDsl, RunQueryDsl};
use diesel::prelude::*;
use crate::article::article_statistics::sql::model::article_statistics::dsl::*;
use crate::article::article_statistics::sql::model::ArticleStatisticsDB;
use crate::utils::sql::sql_conn::get_connection;

pub fn list_article_statistics(article_id_list: Vec<i64>) -> Option<Vec<ArticleStatisticsDB>> {
    let conn = &mut get_connection();

    let query_result = article_statistics
        .filter(article_id.eq_any(article_id_list))
        .load::<ArticleStatisticsDB>(conn);

    return if let Ok(res) = query_result {
        Some(res)
    } else {
        None
    };
}

#[test]
fn test() {
    println!("{:?}", list_article_statistics(vec![1]));
}

