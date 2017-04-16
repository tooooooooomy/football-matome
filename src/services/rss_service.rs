use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use models::feed::Feed;
use schema::feeds::dsl::*;

#[derive(RustcEncodable)]
pub struct ResFeed {
    id: i32,
    title: String,
    link: String,
}

pub fn retrieve(conn: &MysqlConnection) -> Vec<ResFeed>{
    feeds
        .limit(20)
        .load::<Feed>(conn)
        .expect("Error loading feeds")
        .into_iter()
        .map(|feed| make_res_feed_from_feed(feed))
        .collect::<Vec<ResFeed>>()
}

fn make_res_feed_from_feed (feed: Feed) -> ResFeed {
    ResFeed {
        id: feed.id,
        title: feed.title,
        link: feed.link,
    }
}
