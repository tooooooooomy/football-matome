use integration::support::{Body, get};
use hyper::header;
use hyper::status::StatusCode;
use rustc_serialize::json::Json;
use std::env;

#[test]
fn get_empty_feeds() {
    env::set_var("DATABASE_URL", "mysql://root@localhost/test_football_matome");
    let mut response = get("/football-matome/api/get");
    let json = Json::from_str(&response.body()).unwrap();

    assert_eq!(StatusCode::Ok, response.status);
    assert_eq!(response.headers.get::<header::ContentType>(), Some(&header::ContentType::json()));
    assert_eq!(Some(&vec![]), json["data"].as_array());
}
