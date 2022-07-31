use crate::types;
use r2d2_postgres::postgres::{Client, NoTls};
use r2d2_postgres::PostgresConnectionManager;
use std::env;

extern crate lazy_static;

lazy_static::lazy_static! {
    pub static ref POOL: r2d2::Pool<PostgresConnectionManager<NoTls>> = {
        let host = env::var("HOST").unwrap();
        let user = env::var("USER").unwrap();
        let conn_string = match env::var("PASS") {
            Ok(value) => format!(
                "host={} user={} password={} dbname=squawk",
                host, user, value
            ),
            Err(e) => format!("host={} user={} dbname=squawk", host, user),
        };
        let manager = PostgresConnectionManager::new(conn_string.parse().unwrap(), NoTls);
        r2d2::Pool::new(manager).unwrap()
    };
}
