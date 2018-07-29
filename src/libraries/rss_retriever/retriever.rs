extern crate hyper;
extern crate xml;

use self::hyper::Client;
use self::hyper::client::response::Response;
use self::hyper::error::Result;
use self::hyper::status::StatusCode;
use std::io::BufReader;
use self::xml::reader::{EventReader, XmlEvent};

pub struct Retriever {
    url: String
}

impl Retriever {
    pub fn new(url:&str) -> Retriever {
        Retriever {
            url: url.to_string()
        }
    }

    pub fn get_item_list(&self) -> (Vec<String>, Vec<String>) {
        let result = Retriever::get_feed(self.url.as_str());

        match result {
            Ok(v) => {
                match v.status {
                    StatusCode::InternalServerError => panic!("Request failed! {:?}", v),
                    _ => Retriever::extract_item_list(v),
                }
            }
            Err(e) => panic!("Unknown error occurred! {:?}", e),
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
                    panic!("Something wrong! {:?}", e)
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
    use std::fs::File;
    use std::io::prelude::*;
    use std::env;

    const URL: &'static str = mockito::SERVER_URL;

    #[test]
    #[should_panic]
    fn test_get_item_list_when_error() {
        mockito::mock("GET", "/")
            .with_status(500)
            .with_header("content-type", "text/plain")
            .with_body("hogehoge")
            .create();

        let t = super::Retriever::new(URL);

        t.get_item_list();
    }

    #[test]
    fn test_get_item_list() {
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

        let t = super::Retriever::new(URL);

        let result = t.get_item_list();

        let expected = (vec![
            "【ネイマール】30日に緊急来日決定！スポンサー企業の催しに出席しサイン会も実施。".to_string(),
            "【日本代表】選考基準は“ケンカ上等”？「ピッチ状態も今回はキーになる」".to_string(),
            "【ハリル監督】エース交代！就任以来初めて本田を久保に次ぐ2番手として評価！".to_string(),
            "【オーバメヤン】ドルトムント退団を希望と独紙報道！100億円で移籍を容認へ。".to_string(),
            "【日本代表】なぜ本田圭佑は生き残ったのか？常連組が消えたハリルジャパン".to_string(),
            "【画像】今季JリーグGKセーブ率ランキング…1人神がいた！？wwww".to_string(),
            "マンUの堅守速攻が許せない？アヤックス指揮官&クラーセン主将が不満！「引いて、ロングボール蹴るのみ」".to_string(),
            "【画像】日本代表、シリア戦は“炎モデル”…W杯初出場決定20周年メモリアルユニを着用".to_string(),
            "【日本代表】“川口の再来”中村航輔が代表入り！最近の神がかりっぷりがやばい".to_string(),
            "【ネタ動画】CLに出れないアーセナル、ぼろくそでワロタwwwwwwwww".to_string()
        ], vec![
            "http://samuraigoal.doorblog.jp/archives/51364634.html".to_string(),
            "http://samuraigoal.doorblog.jp/archives/51364269.html".to_string(),
            "http://samuraigoal.doorblog.jp/archives/51363949.html".to_string(),
            "http://samuraigoal.doorblog.jp/archives/51363357.html".to_string(),
            "http://samuraigoal.doorblog.jp/archives/51361565.html".to_string(),
            "http://samuraigoal.doorblog.jp/archives/51361065.html".to_string(),
            "http://samuraigoal.doorblog.jp/archives/51361288.html".to_string(),
            "http://samuraigoal.doorblog.jp/archives/51361361.html".to_string(),
            "http://samuraigoal.doorblog.jp/archives/51361175.html".to_string(),
            "http://samuraigoal.doorblog.jp/archives/51354844.html".to_string()
        ]);

        assert_eq!(expected, result);
    }
}
