use diesel::prelude::*;
use dotenvy::dotenv;
use diesel::r2d2::{ConnectionManager, Pool};

/*
sudo apt update
sudo apt install libsqlite3-dev
*/

pub fn create_db_pool() -> Pool<ConnectionManager<SqliteConnection>> {
    dotenv().ok();
    let database_url = "database.db";
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Failed to create database pool")
}