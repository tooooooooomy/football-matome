use chrono;
use schema::feeds;
use diesel;
use diesel::ExecuteDsl;
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
