use diesel::prelude::*;
use models::connection;
use models::feed::Feed;
use schema::feeds::dsl::*;

#[derive(RustcEncodable)]
pub struct ResFeed {
    id: i32,
    title: String,
    link: String,
}

pub fn retrieve() -> Vec<ResFeed>{
    let connection = connection::establish_connection();
    let results = feeds
        .limit(20)
        .load::<Feed>(&connection)
        .expect("Error loading feeds");

    let mut v: Vec<ResFeed> = vec![];
    for row in results {
        let feed = ResFeed {
            id: row.id,
            title: row.title,
            link: row.link,
        };

        v.push(feed);
    }

    v
}
