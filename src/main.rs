use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*, client::ClientBuilder,
};

// HELP MESSAGE
const HELP_MESSAGE: &str = "
Hello there, Human!

You have summoned the RustyBot. Let's see about getting you what you need.

? Need Technical Help ?
=> Post in the <#1021521698115289098> channel and other humans will assist you.

? Looking for the Code of Conduct ?
=> Here it is: <https://discord.com/guidelines>

? Something Wrong ?
=> Flag an admin with @admin

Hope that helps. For more issues, ask a human, I'm just a starter bot.
-- RustyBot
";

// SET HELP COMMAND
const HELP_COMMAND: &str = "!help";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == HELP_COMMAND {
            if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
              println!("Error sending message: {:?}", why);  
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // get the discord token env variable
    let token = env::var("DISCORD_TOKEN")
    .expect("Expected a token in the environment");

    // use token o create new Serenity Client
    let mut client = ClientBuilder::new(&token)
    .event_handler(Handler)
    .await
    .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}