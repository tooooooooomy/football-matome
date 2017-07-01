use chrono;
use diesel;
use diesel::*;
use diesel::mysql::MysqlConnection;
use schema::feeds;

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

pub fn exists(conn: &MysqlConnection, target_title: &str) -> bool {
    use schema::feeds::dsl::*;
    match feeds.filter(title.eq(target_title)).count().get_result(conn) {
        Ok(0) => false,
        Ok(_) => true,
        Err(e) => panic!(e),
    }
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use std::env;
    use models::connection;
    use super::*;
    use schema::feeds::dsl::*;

    #[test]
    fn test_create_feed() {
        dotenv().ok();
        let database_url = env::var("TEST_DATABASE_URL")
            .expect("TEST_DATABASE_URL must be set");
        let connection = connection::establish_connection(&database_url);
        connection.execute("truncate table feeds").unwrap();

        create_feed(&connection, "hoge", "http://hoge.com");

        let record = feeds.first::<Feed>(&connection).unwrap();

        assert_eq!("hoge", record.title);
        assert_eq!("http://hoge.com", record.link);
    }

    #[test]
    fn test_exists() {
        dotenv().ok();
        let database_url = env::var("TEST_DATABASE_URL")
            .expect("TEST_DATABASE_URL must be set");
        let connection = connection::establish_connection(&database_url);
        connection.execute("truncate table feeds").unwrap();
        create_feed(&connection, "hoge", "http://hoge.com");

        assert!(exists(&connection, "hoge"));
        assert_ne!(true, exists(&connection, "fuga"));
    }
}
