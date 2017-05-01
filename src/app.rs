use nickel::{Nickel, ListeningServer, HttpRouter};
use controllers::feed_controller;
use std::error::Error as StdError;

pub fn start_server(address: &str) -> Result<ListeningServer, Box<StdError>>{
    let mut server = Nickel::new();

    server.get("/football-matome/api/get", feed_controller::get_feed);

    server.listen(address)
}

#[cfg(test)]
mod tests {
    use self::support::{Server, STATIC_SERVER, get};
    use hyper::header;
    use nickel::status::StatusCode;
    use rustc_serialize::json::Json;

    #[test]
    fn get_empty_feeds() {
        let mut response = get("/football-matome/api/get");

        assert_eq!(StatusCode::Ok, response.status);
    }

    mod support {
        use hyper::client::{Client, Response as HyperResponse};
        use nickel::ListeningServer;
        use std::net::SocketAddr;

        pub struct Server(SocketAddr);
        impl Server {
            pub fn new(server: ListeningServer) -> Server{
                let wrapped = Server(server.socket());

                server.detach();

                wrapped
            }

            pub fn get(&self, path: &str) -> HyperResponse {
                let url = self.url_for(path);
                Client::new().get(&url).send().unwrap()
            }

            pub fn url_for(&self, path: &str) -> String{
                format!("http://{}{}", self.0, path)
            }
        }

        // This is a shared instance of the server between all the tests
        lazy_static! {
            pub static ref STATIC_SERVER: Server = {
                let server = super::super::start_server("127.0.0.1:0").unwrap();
                Server::new(server)
            };
        }

        pub fn get(path: &str) -> HyperResponse {
            STATIC_SERVER.get(path)
        }
    }
}
