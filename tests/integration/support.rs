use hyper::client::{Client, Response as HyperResponse};
use nickel::ListeningServer;
use std::net::SocketAddr;
use football_matome::app::start_server;

pub trait Body {
    fn body(self) -> String;
}

impl<'a> Body for &'a mut HyperResponse {
    fn body(self) -> String {
        use std::io::Read;
        let mut body = String::new();
        self.read_to_string(&mut body).expect("Failed to read body from response");

        body
    }
}

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
        let server = start_server("127.0.0.1:0").unwrap();
        Server::new(server)
    };
}

pub fn get(path: &str) -> HyperResponse {
    STATIC_SERVER.get(path)
}
