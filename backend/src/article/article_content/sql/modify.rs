use diesel::{QueryResult, RunQueryDsl};
use crate::article::article_content::sql::model::{article_content, ArticleContentDB};
use crate::utils::sql::sql_conn::get_connection;

pub fn insert(article_content_: ArticleContentDB) -> QueryResult<usize> {
    let conn = &mut get_connection();

    diesel::insert_into(article_content::table)
        .values(vec![article_content_])
        .execute(conn)
}

#[test]
fn test_insert() {
    let article_content = ArticleContentDB {
        id: None,
        article_id: 1,
        content: "test article content insert".to_string(),
        status: 0,
        create_time: None,
        modify_time: None,
    };

    match insert(article_content) {
        Ok(n) => {
            println!("{:?}", n);
        },
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
