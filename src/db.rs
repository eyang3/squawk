use crate::types;
use async_once::AsyncOnce;
use bb8;
use bb8_postgres::PostgresConnectionManager;
use r2d2_postgres;
use r2d2_postgres::postgres::{Client, NoTls};
use std::env;
extern crate lazy_static;

lazy_static::lazy_static! {
    pub static ref POOL: r2d2::Pool<r2d2_postgres::PostgresConnectionManager<NoTls>> = {
        let host = env::var("EMSPLACE_HOST").unwrap();
        let user = env::var("EMSPLACE_USER").unwrap();
        let conn_string = match env::var("PASS") {
            Ok(value) => format!(
                "host={} user={} password={} dbname=squawk",
                host, user, value
            ),
            Err(e) => format!("host={} user={} dbname=squawk", host, user),
        };
        let manager =  r2d2_postgres::PostgresConnectionManager::new(conn_string.parse().unwrap(), NoTls);
        let pool = r2d2::Pool::new(manager).unwrap();
        return pool;
    };
}

lazy_static::lazy_static! {
    pub static ref BB8Pool: AsyncOnce<bb8::Pool<bb8_postgres::PostgresConnectionManager<NoTls>>> = AsyncOnce::new(async {
        let host = env::var("EMSPLACE_HOST").unwrap();
        let user = env::var("EMSPLACE_USER").unwrap();
        let conn_string = match env::var("PASS") {
            Ok(value) => format!(
                "host={} user={} password={} dbname=squawk",
                host, user, value
            ),
            Err(e) => format!("host={} user={} dbname=squawk", host, user),
        };
        let manager = bb8_postgres::PostgresConnectionManager::new(conn_string.parse().unwrap(), NoTls);
        let pool = bb8::Pool::builder().build(manager).await.unwrap();
        return pool;
    });
}
