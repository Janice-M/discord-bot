use discord_bot::{generate_random_alias, get_r6_stats, parse_r6_args};
use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }

        if msg.content == "!alias" {
            if let Err(why) = msg.channel_id.say(&ctx.http, generate_random_alias()).await {
                println!("Error sending message: {:?}", why);
            }
        }

        if msg.content == "!gh" {
            if let Err(why) = msg
                .channel_id
                .say(
                    &ctx.http,
                    "Fork me on github: https://github.com/collinsmuriuki/discord-bot",
                )
                .await
            {
                println!("Error sending message: {:?}", why);
            }
        }

        if msg.content == "!slide" {
            let dm = msg
                .author
                .dm(&ctx, |m| {
                    m.content("Hello!");

                    m
                })
                .await;

            if let Err(why) = dm {
                println!("Error when direct messaging user: {:?}", why);
            }
        }

        if msg.content.starts_with("!r6") {
            if let Some((_, username)) = parse_r6_args(&msg.content) {
                let ((_username, _platform)) = parse_r6_args(username).unwrap_or(("thiself", "pc"));
                unimplemented!()
            } else {
                if let Err(why) = msg
                    .channel_id
                    .say(
                        &ctx.http,
                        "Make sure you specify the username then the platform to get stats!",
                    )
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::new(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
