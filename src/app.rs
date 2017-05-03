use nickel::{Nickel, ListeningServer, HttpRouter};
use controllers::feed_controller;
use std::error::Error as StdError;

pub fn start_server(address: &str) -> Result<ListeningServer, Box<StdError>>{
    let mut server = Nickel::new();

    server.get("/football-matome/api/get", feed_controller::get_feed);

    server.listen(address)
}
