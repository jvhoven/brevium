use lazy_static::lazy_static;
use std::default::Default;
use std::sync::RwLock;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Config {
    pub version: u8,
    pub initialized: bool,
    pub backend: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: 0,
            initialized: false,
            backend: String::from("tantivy"),
        }
    }
}

lazy_static! {
    #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
    pub static ref CONFIG: RwLock<Config> = {
        let mut config = Config::default();
        if let Ok(config_file) = confy::load("brevium", "settings") {
            config = config_file;
        }
        RwLock::new(config)
    };
}
