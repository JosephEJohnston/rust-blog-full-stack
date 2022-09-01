use diesel::sql_function;

sql_function!(fn last_insert_id() -> BigInt);
