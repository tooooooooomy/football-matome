extern crate hyper;
extern crate xml;

use self::hyper::Client;
use self::hyper::client::response::Response;
use self::hyper::error::Result;
use std::io::BufReader;
use self::xml::reader::{EventReader, XmlEvent};

pub struct FeedRetriever {
    url: String
}

impl FeedRetriever {
    pub fn new(url:&str) -> FeedRetriever {
        FeedRetriever {
            url: url.to_string()
        }
    }

    pub fn get_item_list(&self) -> (Vec<String>, Vec<String>) {
        let feed = FeedRetriever::get_feed(self.url.as_str());

        match feed {
            Ok(v) => {
                return FeedRetriever::extract_item_list(v);
            }
            Err(e) => {
                return e;
            }
        }
    }

    fn get_feed(url: &str) -> Result<Response> {
        let client = Client::new();
        let response_builder = client.get(url);

        response_builder.send()
    }

    fn extract_item_list(res:Response) -> (Vec<String>, Vec<String>){
        let mut title_list = Vec::new();
        let mut link_list = Vec::new();

        let buf = BufReader::new(res);
        let parser = EventReader::new(buf);

        let mut in_item_elm = false;
        let mut in_title_elm = false;
        let mut in_link_elm = false;

        for elm in parser {
            match elm {
                Ok(XmlEvent::StartElement {name,..}) => {
                    match name.local_name.as_str() {
                        "item" => in_item_elm = true,
                        "title" => in_title_elm = true,
                        "link" => in_link_elm = true,
                        _ => {},
                    }
                }
                Ok(XmlEvent::EndElement{name}) => {
                    match name.local_name.as_str() {
                        "item" => in_item_elm = false,
                        "title" => in_title_elm = false,
                        "link" => in_link_elm = false,
                        _ => {},
                    }
                }
                Ok(XmlEvent::Characters(text)) => {
                    if !in_item_elm {
                        continue;
                    }

                    if in_title_elm {
                        title_list.push(text.trim().to_string());
                    } else if in_link_elm {
                        link_list.push(text.trim().to_string());
                    }
                }
                Err(e) => {
                    println!("Error:{}", e);
                    break;
                }
                _ => {},
            }
        }

        (title_list, link_list)
    }
}

#[cfg(test)]
mod tests {

    use mockito;

    const URL: &'static str = mockito::SERVER_URL;

    fn create_mock() {

        mockito::mock("GET", "/")
            .with_status(500)
            .with_header("content-type", "text/plain")
            .with_header("x-api-key", "1234")
            .with_body("hogehoge")
            .create();
    }

    #[test]
    fn test_get_item_list_when_error() {
        mockito::mock("GET", "/")
            .with_status(500)
            .with_header("content-type", "text/plain")
            .with_header("x-api-key", "1234")
            .with_body("hogehoge")
            .create();

        let t = super::FeedRetriever::new(URL);

        assert_eq!(Err, t.get_item_list());
    }
}
