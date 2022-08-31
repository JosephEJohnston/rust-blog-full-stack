#![allow(dead_code)]

use diesel::prelude::*;
use crate::article::sql::model::ArticleDB;
use crate::sql_conn::get_connection;
use crate::article::sql::model::article;
use crate::article::sql::model::article::dsl::*;

pub fn insert(article: ArticleDB) -> QueryResult<ArticleDB> {
    let conn = &mut get_connection();

    // todo
    diesel::insert_into(article::table)
        .values(vec![article])
        .get_result(conn)
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

        if let Ok(article) = insert(article) {
            println!("Insert article: {:?}", article);
        }
    }
}