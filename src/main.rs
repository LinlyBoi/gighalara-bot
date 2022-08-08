use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        let text = msg.content.to_string();
        let phrase = get_response(&text);
        
        // This code was written by idiot (me)
        // if msg.content == "hello" {
        //     phrase = "http://nohello.com".to_string();
        // }
        // else if msg.content == "help"
        // {
        //     phrase = "https://dontasktoask.com/".to_string();
        // }
        if msg.content == "!help"
        {
            let dm = msg.author.dm(&ctx, |m| m.content("I hate you")).await;
            if let Err(why) = dm {
                println!("Couldn't dm the noob: {:?}", why);
            }
        }
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
        if  &phrase != "not pog"
        {
            if let Err(why) = msg.channel_id.say(&ctx.http, &phrase).await {
                println!("Error sending message: {:?}", why);
            }

        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
fn get_response(txt: &str) -> String {
    match txt {
        "hello" | "hi" | "hey" => return "https://nohello.com".to_string(),
        "help" | "can anyone help?" | "helppp" => return "https://dontasktoask.com".to_string(),
        "arch btw" | "I use arch btw" => return "no one fucking cares".to_string(),
        "I am so good at dragon ball legends" => return "dragon ball fans can't fucking read lmao".to_string(), //friend suggestion 
        "ya ged3an" => return "SHUT THE FUCK UP".to_string(),
        
        _ => return "not pog".to_string(),
    }


}
