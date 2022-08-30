#![allow(dead_code)]

use diesel::{QueryResult, RunQueryDsl};
use crate::article::sql::model::ArticleDB;
use crate::sql_conn::get_connection;
use crate::article::sql::model::article;

pub fn insert(article: ArticleDB) -> QueryResult<usize> {
    let conn = &mut get_connection();

    diesel::insert_into(article::table)
        .values(vec![article])
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
            title: "测试文章插入2".to_string(),
            outline: "测试文章插入2".to_string(),
            status: 0,
            create_time: None,
            modify_time: None
        };

        if let Ok(n) = insert(article) {
            println!("Insert article: {:?}", n);
        }
    }
}