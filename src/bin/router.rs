#[macro_use]
extern crate nickel;
extern crate football_matome;
extern crate dotenv;

use nickel::{Nickel, HttpRouter};
use rustc_serialize::json;
use nickel::status::*;
use nickel::mimes::MediaType;
use dotenv::dotenv;
use std::env;
use football_matome::controllers::feed_controller::get_feed;

#[allow(unreachable_code, unused_variables, resolve_trait_on_defaulted_unit)]
fn main() {
    let mut server = Nickel::new();
    dotenv().ok();

    server.get("/football-matome/api/get", get_feed);

    let api_port = env::var("API_PORT").expect("API_PORT must be set");
    server.listen(&*api_port).unwrap();
}
