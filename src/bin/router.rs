#[macro_use]
extern crate nickel;
extern crate diesel;
extern crate football_matome;
extern crate rustc_serialize;

use nickel::{Nickel, HttpRouter};
use self::football_matome::*;
use diesel::prelude::*;
use rustc_serialize::json;
use nickel::status::*;
use nickel::mimes::MediaType;
use football_matome::models::Feed;

#[derive(RustcDecodable, RustcEncodable)]
pub struct ResFeed {
    id: i32,
    title: String,
    link: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct ResponseBody {
    data: Vec<ResFeed>,
}

fn main() {
    use football_matome::schema::feeds::dsl::*;
    let mut server = Nickel::new();

    server.get("/football-matome/api/get", middleware! { |request, mut response|
        use football_matome::models::Feed;
        let connection = establish_connection();
        let results = feeds
            .limit(10)
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

    server.listen("192.168.33.10:4000").unwrap();
}
