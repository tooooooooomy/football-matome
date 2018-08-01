use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use models::feed;
use models::feed::Feed;
use schema::feeds::dsl::*;
use libraries::rss_retriever::retriever::Retriever;

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

pub fn create_feeds(conn: &MysqlConnection, sources: &Vec<String>) {
    for url in sources.iter() {
        let (title_list, link_list) = Retriever::new(url).get_item_list();

        for (n, t) in title_list.iter().enumerate() {
            println!("{:?}", t);
            if !feed::exists(&conn, t) {
                feed::create(&conn, t, &link_list[n]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use diesel::result::Error;
    use dotenv::dotenv;
    use mockito;
    use models::feed;
    use models::connection;
    use std::env;
    use std::fs::File;
    use std::io::prelude::*;
    use super::*;

    const URL: &'static str = mockito::SERVER_URL;

    fn get_connection() -> MysqlConnection {
        let database_url = env::var("TEST_DATABASE_URL")
            .expect("TEST_DATABASE_URL must be set");

        connection::establish_connection(&database_url)
    }

    #[test]
    fn test_retrieve() {
        dotenv().ok();
        let connection = get_connection();
        connection.execute("truncate table feeds").unwrap();

        connection.test_transaction::<_, Error, _>(|| {
            let title_1 = "hoge";
            let link_1 = "http://hoge.com";
            let title_2 = "fuga";
            let link_2 = "http://fuga.com";

            feed::create(&connection, &title_1, &link_1);
            feed::create(&connection, &title_2, &link_2);

            let result = retrieve(&connection);

            assert_eq!(title_2, result[0].title);
            assert_eq!(title_1, result[1].title);
            assert_eq!(2, result.len());

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

    #[test]
    #[ignore]
    fn test_create_feeds() {
        dotenv().ok();
        let database_url = env::var("TEST_DATABASE_URL")
            .expect("TEST_DATABASE_URL must be set");
        let connection = connection::establish_connection(&database_url);
        connection.test_transaction::<_, Error, _>(|| {
            let current_dir = env::current_dir().unwrap();
            let file_path = format!("{}/tests/stabs/matome.rdf", current_dir.display());
            let mut file = File::open(file_path).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            let _m = mockito::mock("GET", "/")
                .with_status(200)
                .with_header("content-type", "text/xml; charset=utf-8")
                .with_body(&contents)
                .create();

            let sources = vec![URL.to_string()];

            create_feeds(&connection, &sources);
            assert_eq!(Ok(10), feeds.count().get_result(&connection));
            Ok(())
        })
   }
}
