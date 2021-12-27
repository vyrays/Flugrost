# Flugrost
Small discord bot using an open source weather API, written in Rust.

The only requirement to use this would be a discord bot token, that needs to be put into the config file from which the application reads.
*A future approach might be to provide this token through environment variables or as argument for the executable.*

# Open Source Weather API
Credits go to [wttr.in](https://github.com/chubin/wttr.in).

# Dependencies
The dependencies used in this repository are:
* [Serenity](https://github.com/serenity-rs/serenity), which launches the discord bot
* [Tokio](https://github.com/tokio-rs/tokio), giving Rust an awesome runtime for web
* [Serde](https://github.com/serde-rs/serde), makes my life easier when dealing with JSON data
* [Reqwest](https://github.com/seanmonstar/reqwest), allows me to just point at a certain location at the web easily
