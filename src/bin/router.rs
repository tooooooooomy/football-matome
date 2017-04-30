extern crate football_matome;
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use football_matome::app;

fn main() {
    dotenv().ok();
    let api_port = env::var("API_PORT").expect("API_PORT must be set");
    app::start_server(&api_port).unwrap();
}
