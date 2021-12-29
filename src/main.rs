mod config;

use crate::config::config::ConfigTrait;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    utils::MessageBuilder,
};

#[derive(Debug)]
enum ConfigHandler {
    File(config::file::Config),
    Argument(config::argument::Config),
    Environment(config::environment::Config),
}

impl ConfigHandler {
    fn get_token(self) -> String {
        match self {
            ConfigHandler::File(file) => file.token,
            ConfigHandler::Argument(argument) => argument.token,
            ConfigHandler::Environment(environment) => environment.token,
        }
    }
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        // Make sure the bot doesn't read it's own messages.
        if !msg.author.bot && msg.content.to_lowercase().contains("wetter") {
            let channel = match msg.channel_id.to_channel(&context).await {
                Ok(channel) => channel,
                Err(why) => {
                    println!("Error getting channel: {:?}", why);
                    return;
                }
            };

            // @TODO: Channel name changeable/scope-able through config (mutex/arc).

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

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
