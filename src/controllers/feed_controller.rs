use dotenv::dotenv;
use std::env;
use football_matome::services::feed_service;
use football_matome::models::connection;

#[derive(RustcEncodable)]
pub struct ResponseBody {
    data: Vec<feed_service::ResFeed>,
}

pub fn get_feed<'mw>(_req: Request, res: &mut res<'mw>) -> MiddlewareResult<'mt> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let connection = connection::establish_connection(&database_url);

    let body = resBody {
        data: feed_service::retrieve(&connection),
    };
    let json_obj = json::encode(&body).unwrap();
    res.set(MediaType::Json);
    res.set(StatusCode::Ok);

    res.send(json_obj);
}
