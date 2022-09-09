#![allow(dead_code)]

use diesel::{QueryResult, RunQueryDsl};
use crate::utils::sql::sql_conn::get_connection;
use crate::user::sql::model::user;
use crate::user::sql::model::UserDB;

fn insert(user: UserDB) -> QueryResult<usize> {
    let conn = &mut get_connection();

    diesel::insert_into(user::table)
        .values(vec![user])
        .execute(conn)
}


#[cfg(test)]
mod tests {
    use crate::user::sql::model::UserDB;
    use crate::user::sql::modify::insert;

    #[test]
    fn test_insert() {
        let user = UserDB {
            id: None,
            name: "Theresia".to_string(),
            email: "1193347519@qq.com".to_string(),
            phone: "18760257158".to_string(),
            password: "731320092".to_string(),
            encrypted_password: "".to_string(),
            status: 1,
            create_time: None,
            modify_time: None,
        };

        if let Ok(n) = insert(user) {
            println!("Insert done: {:?}", n);
        }
    }
}