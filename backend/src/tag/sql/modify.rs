#![allow(dead_code)]

use diesel::{QueryResult, RunQueryDsl};
use crate::utils::sql::sql_conn::get_connection;
use crate::tag::sql::model::{tag, TagDB};

fn insert(tag: &TagDB) -> QueryResult<usize> {
    let conn = &mut get_connection();

    diesel::insert_into(tag::table)
        .values(vec![tag])
        .execute(conn)
}

#[test]
fn test_insert() {
    let tag = TagDB {
        id: None,
        user_id: 1,
        name: "init-2".to_string(),
        status: 0,
        create_time: None,
        modify_time: None,
    };

    if let Ok(n) = insert(&tag) {
        println!("Insert tag: {:?}", n);
        println!("{:?}", tag.id);
    }
}
