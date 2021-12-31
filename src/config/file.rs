use crate::config::config::ConfigTrait;
use crate::ConfigHandler;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::{error, fmt};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

struct TokenNotFoundError;

impl fmt::Display for TokenNotFoundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to find a valid token.")
    }
}

impl Debug for TokenNotFoundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to find a valid token.")
    }
}

impl error::Error for TokenNotFoundError {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub channel: Option<String>,
    pub token: String,
    pub command: String,
}

impl ConfigTrait for Config {
    fn new() -> Result<ConfigHandler> {
        let config_string: String = std::fs::read_to_string("src/config.json")?;
        let mut config_json: Config = serde_json::from_str(&config_string)?;
        if config_json.token == "" {
            return Err(Box::new(TokenNotFoundError));
        }
        config_json.channel = config_json.channel.and_then(|channel| {
            if channel == String::from("") {
                return None;
            }
            Some(channel)
        });
        // Always have at least "wetter" as command. Otherwise, take the command from the config.
        config_json.command = {
            if config_json.command != "" {
                config_json.command
            } else {
                String::from("wetter")
            }
        };
        Ok(ConfigHandler::File(Config {
            channel: config_json.channel,
            token: config_json.token,
            command: config_json.command,
        }))
    }
}
