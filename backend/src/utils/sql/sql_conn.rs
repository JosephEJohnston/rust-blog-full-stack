#![allow(dead_code)]

use diesel::prelude::*;
use std::env;
use std::time::Duration;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use dotenvy::dotenv;

pub fn get_connection() -> PooledConnection<ConnectionManager<MysqlConnection>>
{
    let pool = get_connection_pool().clone();

    let conn = pool
        .get_timeout(Duration::new(5, 0))
        .unwrap();

    conn
}

fn get_connection_pool() -> Pool<ConnectionManager<MysqlConnection>> {
    dotenv().ok();

    let url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<MysqlConnection>::new(url);

    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

