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
use football_matome::services::feed_service;
use football_matome::models::connection;

#[derive(RustcEncodable)]
pub struct ResponseBody {
    data: Vec<feed_service::ResFeed>,
}

#[allow(unreachable_code, unused_variables, resolve_trait_on_defaulted_unit)]
fn main() {
    let mut server = Nickel::new();
    dotenv().ok();

    server.get("/football-matome/api/get", middleware! { |request, mut response|
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        let connection = connection::establish_connection(&database_url);

        let body = ResponseBody {
            data: feed_service::retrieve(&connection),
        };
        let json_obj = json::encode(&body).unwrap();
        response.set(MediaType::Json);
        response.set(StatusCode::Ok);
        return response.send(json_obj);
    });

    let api_port = env::var("API_PORT").expect("API_PORT must be set");
    server.listen(&*api_port).unwrap();
}
