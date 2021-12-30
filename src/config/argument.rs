use crate::config::config::ConfigTrait;
use crate::ConfigHandler;
use serde::de::StdError;
use std::env::args;
use std::error;
use std::fmt::{Display, Formatter};
use std::str::Split;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct Config {
    pub channel: Option<String>,
    pub token: String,
}

#[derive(Debug, Clone)]
struct ConfigError;

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unable to locate token string. Aborting now.")
    }
}

impl StdError for ConfigError {}

impl ConfigTrait for Config {
    fn new() -> Result<ConfigHandler> {
        let arguments = args();
        let filtered_arguments: Vec<String> = arguments
            .filter(|arg| {
                arg.to_lowercase().contains("token") || arg.to_lowercase().contains("channel")
            })
            .collect();
        let mut token_argument: Option<String> = None;
        let mut channel_argument: Option<String> = None;
        for arg in filtered_arguments.iter() {
            let mut split_args: Split<&str> = arg.split("=");
            if let Some(str) = split_args.next() {
                if str.contains("token") {
                    token_argument = split_args.next().map(|token| String::from(token));
                } else if str.contains("channel") {
                    channel_argument = split_args.next().map(|channel| String::from(channel));
                }
            }
        }

        if token_argument == None {
            return Err(Box::new(ConfigError));
        }

        Ok(ConfigHandler::Argument(Config {
            channel: channel_argument,
            token: token_argument.unwrap(),
        }))
    }
}
