use std::env;
use services::feed_service;
use models::connection;
use nickel::{Request, Response, MiddlewareResult};
use nickel::mimes::MediaType;
use nickel::status::*;
use rustc_serialize::json;

#[derive(RustcEncodable)]
pub struct ResponseBody {
    data: Vec<feed_service::ResFeed>,
}

pub fn get_feed<'mw>(_req: &mut Request, mut res: Response<'mw>) -> MiddlewareResult<'mw> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let connection = connection::establish_connection(&database_url);

    let body = ResponseBody {
        data: feed_service::retrieve(&connection),
    };

    let json_obj = json::encode(&body).unwrap();
    res.set(MediaType::Json);
    res.set(StatusCode::Ok);

    res.send(json_obj)
}
