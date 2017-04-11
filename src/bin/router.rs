#[macro_use]
extern crate nickel;
extern crate football_matome;
extern crate rustc_serialize;
extern crate dotenv;

use nickel::{Nickel, HttpRouter};
use rustc_serialize::json;
use nickel::status::*;
use nickel::mimes::MediaType;
use dotenv::dotenv;
use std::env;
use football_matome::services::rss_service;

#[derive(RustcEncodable)]
pub struct ResponseBody {
    data: Vec<rss_service::ResFeed>,
}

fn main() {
    use football_matome::schema::feeds::dsl::*;
    let mut server = Nickel::new();

    server.get("/football-matome/api/get", middleware! { |request, mut response|
        let body = ResponseBody {
            data: rss_service::retrieve(),
        };
        let json_obj = json::encode(&body).unwrap();
        response.set(MediaType::Json);
        response.set(StatusCode::Ok);
        return response.send(json_obj);
    });

    dotenv().ok();
    let api_port = env::var("API_PORT").expect("API_PORT must be set");
    server.listen(api_port).unwrap();
}
