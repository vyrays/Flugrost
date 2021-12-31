# Flugrost
Small discord bot using an open source weather API, written in Rust.

The only requirement to use this would be a discord bot token, that needs to be put into the config file from which the application reads.
*A future approach might be to provide this token through environment variables or as argument for the executable.*

Works only for guild channels (right now).

# How to make it run
To compile the application, you first have to be in possession of the rust compiler.
That can be accomplished by downloading from [here](https://www.rust-lang.org/tools/install).  
Follow the instructions to properly install all the tools you need (Windows requires some additional steps).  
After successfully installing the rust compiler, run the application with
```
run --package flugrost --bin flugrost
```

# Config definition
There are multiple ways of defining the configuration that contains all the necessary or optional information.
### Config as JSON file
To use a JSON file as config simply take the config.json, which is provided in the *src* folder, as template:
* **channel** and **command** are optional (whereas **command** will be replaced with "wetter" if not filled out).
* **token** *must* be provided and requires your desired Discord Bot-Token.

### Config as executable arguments
The configuration can also be set as executable arguments. For example  
```run --package flugrost --bin flugrost token=provide_your_token_here channel=weather command=weather```  
would be valid to run the bot, that will listen to *weather London* and will only respond if the user sent that in the 'weather' channel.

### Config as environment variables
Applying the config with environment variables is similar to the configuration with executable arguments.
The environment variables available (right now) are:
* TOKEN (required)
* CHANNEL (optional)
* COMMAND (optional)

# Open Source Weather API
Credits go to [wttr.in](https://github.com/chubin/wttr.in).

# Dependencies
The dependencies used in this repository are:
* [Serenity](https://github.com/serenity-rs/serenity), which launches the discord bot
* [Tokio](https://github.com/tokio-rs/tokio), giving Rust an awesome runtime for web
* [Serde](https://github.com/serde-rs/serde), makes my life easier when dealing with JSON data
* [Reqwest](https://github.com/seanmonstar/reqwest), allows me to just point at a certain location at the web easily
