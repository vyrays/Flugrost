use crate::config::config::ConfigTrait;
use crate::ConfigHandler;
use serde::{Deserialize, Serialize};
use std::error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub channel: Option<String>,
    pub token: String,
}

impl ConfigTrait for Config {
    fn new() -> Result<ConfigHandler> {
        let config_string: String = std::fs::read_to_string("src/config.json")?;
        let config_json: Config = serde_json::from_str(&config_string)?;
        Ok(ConfigHandler::File(Config {
            channel: config_json.channel,
            token: config_json.token,
        }))
    }
}
