use std::fs::File;
use std::io::prelude::*;
use std::env;
use toml;

#[cfg(not(test))]
const CONFIG_PATH: &'static str = "src/config/config.toml";

#[cfg(test)]
const CONFIG_PATH: &'static str = "tests/stabs/config.toml";

#[derive(Debug, Deserialize)]
pub struct Config {
    sources: Vec<String>,
}

impl Config {
    pub fn new(path: &str) -> Config {
        let current_dir = env::current_dir().unwrap();
        let file_path = format!("{}/{}", current_dir.display(), path);
        let mut file = File::open(file_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        toml::from_str(&contents).unwrap()
    }

    pub fn get_sources(&self) -> &Vec<String> {
        &self.sources
    }
}

lazy_static! {
    pub static ref STATIC_CONFIG: Config = {
        Config::new(CONFIG_PATH)
    };
}

pub fn get_sources() -> &'static Vec<String> {
    STATIC_CONFIG.get_sources()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_get_sources() {
        let expected = &vec![
            "http://samuraigoal.doorblog.jp/index.rdf".to_string(),
        ];

        assert_eq!(expected, super::get_sources());
    }
}
