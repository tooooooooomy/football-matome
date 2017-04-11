extern crate football_matome;

use football_matome::libraries::rss_retriever::feed::Feed;
use football_matome::models::connection;
use football_matome::models::feed;

fn main() {
    let connection = connection::establish_connection();

    let url = "http://samuraigoal.doorblog.jp/index.rdf";

    let f = Feed::new(url);

    let (title_list, link_list) = f.get_item_list();

    for (n, title) in title_list.iter().enumerate() {
        println!("{}\n{}\n", title, link_list[n]);
        feed::create_feed(&connection, title, &link_list[n]);
    }
}
