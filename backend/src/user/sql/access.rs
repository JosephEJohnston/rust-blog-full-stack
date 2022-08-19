use diesel::{QueryResult, RunQueryDsl};
use diesel::sql_types::Varchar;
use crate::sql_conn::get_connection;
use crate::user::sql::model::UserDB;

fn get(email: String) -> QueryResult<UserDB> {
    let conn = &mut get_connection();

    diesel::sql_query("SELECT * FROM user WHERE email = ? AND status = 0 LIMIT 1")
        .bind::<Varchar, _>(email.clone())
        .get_result(conn)
}

#[cfg(test)]
mod tests {
    use crate::user::sql::access::get;

    #[test]
    fn test() {
        if let Ok(user) = get("1193347519@qq.com".to_string()) {
            println!("Get: {:?}", user);
        }
    }
}