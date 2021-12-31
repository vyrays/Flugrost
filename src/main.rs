mod config;

use crate::config::config::ConfigTrait;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    utils::MessageBuilder,
};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
enum ConfigHandler {
    File(config::file::Config),
    Argument(config::argument::Config),
    Environment(config::environment::Config),
}

impl ConfigHandler {
    fn get_token(&self) -> String {
        match &self {
            ConfigHandler::File(file) => file.token.to_owned(),
            ConfigHandler::Argument(argument) => argument.token.to_owned(),
            ConfigHandler::Environment(environment) => environment.token.to_owned(),
        }
    }

    fn get_channel(&self) -> Option<String> {
        match &self {
            ConfigHandler::File(file) => file.channel.to_owned(),
            ConfigHandler::Argument(argument) => argument.channel.to_owned(),
            ConfigHandler::Environment(environment) => environment.channel.to_owned(),
        }
    }

    fn get_command(&self) -> String {
        match &self {
            ConfigHandler::File(file) => file.command.to_owned(),
            ConfigHandler::Argument(argument) => argument.command.to_owned(),
            ConfigHandler::Environment(environment) => environment.command.to_owned(),
        }
    }
}

impl TypeMapKey for ConfigHandler {
    type Value = Arc<HashMap<String, String>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        // Make sure the bot doesn't read it's own messages.
        if msg.author.bot {
            return;
        }

        let data_read = context.data.read().await;
        let map = match data_read.get::<ConfigHandler>() {
            Some(arc) => arc,
            None => return,
        };

        let mut weather_command: &str = "wetter";
        if let Some(command) = map.get("command") {
            weather_command = command.as_str();
        }

        if msg.content.to_lowercase().contains(weather_command) {
            let channel = match msg.channel_id.to_channel(&context).await {
                Ok(channel) => channel,
                Err(why) => {
                    println!("Error getting channel: {:?}", why);
                    return;
                }
            };

            if let Some(chan) = map.get("channel") {
                if channel.guild().unwrap().name != *chan {
                    return;
                }
            }

            // Is the amount of arguments appropriate? There should only be the command and the location.
            let location: Vec<&str> = msg.content.split(" ").collect();
            if location.len() > 2 {
                let response = MessageBuilder::new().push("Too many arguments.").build();
                if let Err(why) = msg.channel_id.say(&context.http, &response).await {
                    println!("Error sending message: {:?}", why);
                }
                return;
            } else if location.len() < 2 {
                let response = MessageBuilder::new()
                    .push("Too few arguments: Location is missing.")
                    .build();
                if let Err(why) = msg.channel_id.say(&context.http, &response).await {
                    println!("Error sending message: {:?}", why);
                }
                return;
            }

            let weather: String = reqwest::get(format!(
                "https://wttr.in/{}?format='%l:+%c+%t(%f)+%h+%w",
                location.iter().next().unwrap().to_owned()
            ))
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

            let response = MessageBuilder::new().push(weather).build();
            if let Err(why) = msg.channel_id.say(&context.http, &response).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn create_config_handler() -> Option<ConfigHandler> {
    // Check if a config file is available.
    if let Ok(config) = config::file::Config::new() {
        return Some(config);
    }
    if let Ok(config) = config::argument::Config::new() {
        return Some(config);
    }
    if let Ok(config) = config::environment::Config::new() {
        return Some(config);
    }

    None
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let config_handler: ConfigHandler =
        create_config_handler().expect("No config handler could be initialized.");
    let mut client = Client::builder(config_handler.get_token())
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert(String::from("command"), config_handler.get_command());
        if let Some(channel) = config_handler.get_channel() {
            map.insert(String::from("channel"), channel);
            data.insert::<ConfigHandler>(Arc::new(map));
        }
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
