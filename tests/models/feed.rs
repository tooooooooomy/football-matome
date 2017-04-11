use football_matome::models::connection;
use football_matome::models::feed;
use football_matome::schema::feeds::dsl::*;
use dotenv::dotenv;
use std::env;
use diesel;
use diesel::ExecuteDsl;
use diesel::LoadDsl;

#[test]
fn create_feed() {
    dotenv().ok();
    let database_url = env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must be set");

    let connection = connection::establish_connection(&database_url);
    let title_1 = "hoge";
    let link_1 = "http://google.com";

    feed::create_feed(&connection, &title_1, &link_1);

    let record = feeds.first::<feed::Feed>(&connection).expect("error");

    assert_eq!(title_1, record.title);
    assert_eq!(link_1, record.link);

    let num_deleted = diesel::delete(feeds)
        .execute(&connection)
        .expect("Error deleting posts");

    assert_eq!(1, num_deleted);
}
