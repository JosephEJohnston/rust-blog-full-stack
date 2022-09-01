#![allow(dead_code)]

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