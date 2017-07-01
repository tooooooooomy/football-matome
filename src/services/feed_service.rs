use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use models::feed::Feed;
use schema::feeds::dsl::*;

#[derive(RustcEncodable, Debug, PartialEq)]
pub struct ResFeed {
    id: i32,
    title: String,
    link: String,
}

pub fn retrieve(conn: &MysqlConnection) -> Vec<ResFeed>{
    feeds
        .limit(100)
        .order(id.desc())
        .load::<Feed>(conn)
        .expect("Error loading feeds")
        .into_iter()
        .map(|feed| make_res_feed_from_feed(feed))
        .collect::<Vec<ResFeed>>()
}

fn make_res_feed_from_feed(feed: Feed) -> ResFeed {
    ResFeed {
        id: feed.id,
        title: feed.title,
        link: feed.link,
    }
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use std::env;
    use diesel::result::Error;
    use models::feed;
    use models::connection;
    use chrono::prelude::*;
    use super::*;

    #[test]
    fn test_retrieve() {
        dotenv().ok();
        let database_url = env::var("TEST_DATABASE_URL")
            .expect("TEST_DATABASE_URL must be set");

        let connection = connection::establish_connection(&database_url);
        connection.execute("truncate table feeds").unwrap();

        connection.test_transaction::<_, Error, _>(|| {
            let title_1 = "hoge";
            let link_1 = "http://hoge.com";
            let title_2 = "fuga";
            let link_2 = "http://fuga.com";

            feed::create_feed(&connection, &title_1, &link_1);
            feed::create_feed(&connection, &title_2, &link_2);

            let result = retrieve(&connection);

            let expected: Vec<ResFeed> = vec![ ResFeed {
                id: 2,
                title: title_2.to_string(),
                link: link_2.to_string(),
            }, ResFeed {
                id: 1,
                title: title_1.to_string(),
                link: link_1.to_string(),
            }];

            assert_eq!(expected, result);

            Ok(())
        })
    }

    #[test]
    fn test_make_res_feed_from_feed() {

        let expected = ResFeed {
            id: 1,
            title: "hoge".to_string(),
            link: "http://hoge.com".to_string(),
        };

        let result = make_res_feed_from_feed(Feed {
            id: 1,
            title: "hoge".to_string(),
            link: "http://hoge.com".to_string(),
            created_at: UTC::now().naive_utc(),
            updated_at: UTC::now().naive_utc(),
        });

        assert_eq!(expected, result);
    }
}
