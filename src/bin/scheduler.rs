extern crate football_matome;
extern crate dotenv;
extern crate schedule;
extern crate chrono;

use schedule::{Agenda, Job};
use football_matome::config::config;
use football_matome::models::connection;
use football_matome::services::feed_service;

fn main() {
    dotenv::dotenv().ok();
    let mut a = Agenda::new();

    a.add(Job::new(|| {
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        let connection = connection::establish_connection(&database_url);
        let sources = config::get_sources().iter().map(|url| url.to_string()).collect::<Vec<String>>();

        feed_service::create_feeds(&connection, &sources);
    }, "0 0 * * * *".parse().unwrap()));

    loop {
        a.run_pending();
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
