#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;
use self::models::{Feed, NewFeed};

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_feed(conn: &MysqlConnection, title:&str, link:&str) {
    use schema::feeds;

    let new_feed = NewFeed {
        title: title,
        link: link,
    };

    diesel::insert(&new_feed).into(feeds::table)
        .execute(conn)
        .expect("Error saving new feed");
}
