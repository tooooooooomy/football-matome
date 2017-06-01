use std::fs::File;
use std::io::prelude::*;
use std::env;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    sources: Vec<String>,
}

impl Config {
    pub fn new() -> Config {
        let current_dir = env::current_dir().unwrap();
        let file_path = format!("{}/src/config/config.toml", current_dir.display());
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
        Config::new()
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
