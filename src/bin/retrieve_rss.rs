extern crate football_matome;
extern crate dotenv;

use football_matome::config::config;
use football_matome::models::connection;
use football_matome::services::feed_service;
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let connection = connection::establish_connection(&database_url);
    let sources = config::get_sources().iter().map(|url| url.to_string()).collect::<Vec<String>>();

    feed_service::create_feeds(&connection, &sources);
}

