use diesel::{QueryResult, RunQueryDsl};
use crate::article::article_statistics::sql::model::{article_statistics, ArticleStatisticsDB};
use crate::utils::sql::sql_conn::get_connection;

pub fn insert_article_statistics(article_statistics_: ArticleStatisticsDB) -> QueryResult<usize> {
    let conn = &mut get_connection();

    diesel::insert_into(article_statistics::table)
        .values(vec![article_statistics_])
        .execute(conn)
}

