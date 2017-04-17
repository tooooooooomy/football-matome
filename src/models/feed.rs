use chrono;
use schema::feeds;
use diesel;
use diesel::*;
use diesel::mysql::MysqlConnection;

#[derive(Queryable)]
pub struct Feed {
    pub id: i32,
    pub title: String,
    pub link: String,
    pub created_at: chrono::prelude::NaiveDateTime,
    pub updated_at: chrono::prelude::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="feeds"]
struct NewFeed<'a> {
    pub title: &'a str,
    pub link: &'a str,
}

pub fn create_feed(conn: &MysqlConnection, title: &str, link: &str) {

    let new_feed = NewFeed {
        title: title,
        link: link,
    };

    diesel::insert(&new_feed).into(feeds::table)
        .execute(conn)
        .expect("Error saving new feed");
}

#[test]
fn test_create_feed() {
    use dotenv::dotenv;
    use std::env;
    use models::connection;
    use schema::feeds::dsl::*;

    dotenv().ok();
    let database_url = env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must be set");

    let connection = connection::establish_connection(&database_url);
    connection.execute("truncate table feeds;").unwrap();

    create_feed(&connection, "hoge", "http://hoge.com");

    let record = feeds.first::<Feed>(&connection).unwrap();

    assert_eq!(1, record.id);
    assert_eq!("hoge", record.title);
    assert_eq!("http://hoge.com", record.link);
}
