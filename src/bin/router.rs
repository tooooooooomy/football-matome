#[macro_use]
extern crate nickel;
extern crate diesel;
extern crate football_matome;
extern crate rustc_serialize;
extern crate dotenv;

use nickel::{Nickel, HttpRouter};
use diesel::prelude::*;
use rustc_serialize::json;
use nickel::status::*;
use nickel::mimes::MediaType;
use dotenv::dotenv;
use std::env;

#[derive(RustcEncodable)]
pub struct ResFeed {
    id: i32,
    title: String,
    link: String,
}

#[derive(RustcEncodable)]
pub struct ResponseBody {
    data: Vec<ResFeed>,
}

fn main() {
    use football_matome::schema::feeds::dsl::*;
    let mut server = Nickel::new();

    server.get("/football-matome/api/get", middleware! { |request, mut response|
        use football_matome::models::feed::Feed;
        let connection = football_matome::models::connection::establish_connection();
        let results = feeds
            .limit(20)
            .load::<Feed>(&connection)
            .expect("Error loading feeds");

        let mut v: Vec<ResFeed> = vec![];
        for row in results {
            let feed = ResFeed {
                id: row.id,
                title: row.title,
                link: row.link,
            };

            v.push(feed);
        }

        let body = ResponseBody {
            data: v,
        };
        let json_obj = json::encode(&body).unwrap();
        response.set(MediaType::Json);
        response.set(StatusCode::Ok);
        return response.send(json_obj);
    });

    dotenv().ok();
    let api_port = env::var("API_ADDRESS").expect("API_ADDRESS must be set");
    server.listen(api_port).unwrap();
}
