extern crate football_matome;
extern crate dotenv;

use football_matome::config::config;
use football_matome::libraries::rss_retriever::retriever::Retriever;
use football_matome::models::connection;
use football_matome::models::feed;
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let connection = connection::establish_connection(&database_url);

    let sources = config::get_sources();

    for url in sources.iter() {
        let f = Retriever::new(url);

        let (title_list, link_list) = f.get_item_list();

        for (n, title) in title_list.iter().enumerate() {
            println!("{}\n{}\n", title, link_list[n]);
            feed::create_feed(&connection, title, &link_list[n]);
        }
    }
}

