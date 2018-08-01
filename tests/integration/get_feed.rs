use integration::support::{Body, get};
use hyper::header;
use hyper::status::StatusCode;
use rustc_serialize::json::Json;
use std::env;
use diesel::Connection;
use diesel::mysql::MysqlConnection;
use diesel::result::Error;
use football_matome::models::connection;

fn get_connection() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    connection::establish_connection(&database_url)
}

#[test]
fn get_empty_feeds() {
    env::set_var("DATABASE_URL", "mysql://root@localhost/test_football_matome");
    let connection = get_connection();
    connection.test_transaction::<_, Error, _>(|| {
        let mut response = get("/football-matome/api/get");
        let json = Json::from_str(&response.body()).unwrap();

        assert_eq!(StatusCode::Ok, response.status);
        assert_eq!(response.headers.get::<header::ContentType>(), Some(&header::ContentType::json()));
        assert_eq!(Some(&vec![]), json["data"].as_array());

        Ok(())
    })
}
