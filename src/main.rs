use std::env;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
struct Handler;

#[async_trait]
impl EventHandler for Handler 
{

    //function executed when new message sent
    async fn message(&self, ctx: Context, msg: Message) 
    {
        let text = msg.content.to_string();
        let phrase = get_response(&text);
        
        if msg.content == "!help"
        {
            let dm = msg.author.dm(&ctx, |m| m.content("I hate you")).await;
            if let Err(why) = dm 
            {
                println!("Couldn't dm the noob: {:?}", why);
            }
        }

        if  &phrase != "not pog"
        {
            //Sending a message can fail, have to handle exceptions 
            if let Err(why) = msg.channel_id.say(&ctx.http, &phrase).await 
            {
                println!("Error sending message: {:?}", why);
            }

        }
        
        //Holy fuck this code fucking sucks how do I sanitise this PLS HELP THERES NO OBJECTS
        // if &text.to_lowercase() == "pls embeb"
        // {
        //     if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| 
        //     {
        //         m.content("this is an embeb")
        //             .tts(true)
        //             .embed( |e| 
        //                     e.title("Really cool title")
        //                     .description("This will explode")
        //                     .field("Wow rust is really cool","Idk what I'm doing",false,)
        //                 )
        //     }
        //     ).await 
        //     {
        //         println!("Error emebbing : {:?}",why);
        //     }
        //     
        // }
        // Hopefully a better refactored version of above function only here to showcase how awful
        // I was
        if &text.to_lowercase() == "pls embed fr fr"
        {
          
            embedded_message(&ctx,&msg,"Wowowowow","Twitch sub notif!","woolHehe woolHoho","!momentummod");
        }

        


        
        
    }

    //Handles the upon login event
    async fn ready(&self, _: Context, ready: Ready)
    {
        println!("{} is connected!", ready.user.name);
    }
}

fn get_response(txt: &str) -> String 
{
    //hard coded for now but will change later pls no bully
    const CRING_WORDS: [&str;4] = ["arch btw","garuda","dragon ball legends", "roblox"];
    for cring in CRING_WORDS 
    { 
        if txt.to_lowercase().contains(&cring) 
        {
            return "you just posted cringe".to_string();
        } 
    }
    match txt.to_ascii_lowercase().as_str() 
    {
        "hello" | "hi" | "hey" => return "https://nohello.com".to_string(),
        "help" | "can anyone help?" | "helppp" => return "https://dontasktoask.com".to_string(),
        _ => return "not pog".to_string(),
    }
}
//Embed function starts here (will likely break everything)
// fn create_embed(title: &str , description: &str, field1: &str , field2: &str ) -> CreateEmbed
// {
//     return |e| CreateEmbed
//     {
//         e.title(&title).description(&description).field(&field1,&field2,false,)
//     }
// }
//It did break everything
async fn embedded_message(ctx:&Context, msg: &Message,title: &str, description: &str, field1: &str, field2: &str )
{
 
    if let Err(why) = msg.channel_id.send_message(&ctx.http,|m|
            {
                m.content("").tts(false).embed( |e| e.title(&title.to_string()).description(&description.to_string()).field(&field1.to_string(),&field2.to_string(),false,))
            }).await{ println!("hello {:?}",why); }

}

//Purely for debugging
fn print_type_of<T>(_: &T) 
{
    println!("{}", std::any::type_name::<T>())
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
