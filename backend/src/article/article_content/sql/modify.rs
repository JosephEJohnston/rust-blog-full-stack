use diesel::{QueryDsl, QueryResult, RunQueryDsl};
use diesel::prelude::*;
use crate::article::article_content::sql::model::{ArticleContentDB};
use crate::article::article_content::sql::model::article_content::{article_id, content};
use crate::article::article_content::sql::model::article_content::dsl::article_content;
use crate::utils::sql::sql_conn::get_connection;

pub fn insert_article_content(article_content_: ArticleContentDB) -> QueryResult<usize> {
    let conn = &mut get_connection();

    diesel::insert_into(article_content)
        .values(vec![article_content_])
        .execute(conn)
}

pub fn update_article_content_sql(article_content_: ArticleContentDB) -> QueryResult<usize> {
    let conn = &mut get_connection();

    diesel::update(article_content
        .filter(article_id.eq(article_content_.article_id)))
        .set(
            content.eq(article_content_.content)
        )
        .execute(conn)
}

#[test]
fn test_insert() {
    let article_content_ = ArticleContentDB {
        id: None,
        article_id: 1,
        content: "test article content insert".to_string(),
        status: 0,
        create_time: None,
        modify_time: None,
    };

    match insert_article_content(article_content_) {
        Ok(n) => {
            println!("{:?}", n);
        },
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
