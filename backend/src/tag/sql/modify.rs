#![allow(dead_code)]

use diesel::{QueryResult, RunQueryDsl};
use crate::sql_conn::get_connection;
use crate::tag::sql::model::{tag, TagDB};

fn insert(tag: TagDB) -> QueryResult<usize> {
    let conn = &mut get_connection();

    diesel::insert_into(tag::table)
        .values(vec![tag])
        .execute(conn)
}

#[cfg(test)]
mod tests {
    use crate::tag::sql::model::TagDB;
    use crate::tag::sql::modify::insert;

    #[test]
    fn test_insert() {
        let tag = TagDB {
            id: None,
            user_id: 1,
            name: "init".to_string(),
            status: 0,
            create_time: None,
            modify_time: None,
        };

        if let Ok(n) = insert(tag) {
            println!("Insert tag: {:?}", n);
        }
    }
}