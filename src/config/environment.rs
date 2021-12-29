use crate::config::config::ConfigTrait;
use crate::ConfigHandler;
use std::env;
use std::error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct Config {
    pub channel: Option<String>,
    pub token: String,
}

impl ConfigTrait for Config {
    fn new() -> Result<ConfigHandler> {
        let token = match env::var("TOKEN") {
            Ok(var) => var,
            Err(err) => return Err(Box::new(err)),
        };

        let channel = match env::var("CHANNEL") {
            Ok(var) => Some(var),
            Err(_err) => None,
        };

        Ok(ConfigHandler::Environment(Config {
            channel,
            token,
        }))
    }
}
