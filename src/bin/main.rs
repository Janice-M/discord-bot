use discord_bot::{generate_random_alias, get_r6_stats, parse_args};
use std::env;

use qrcode::render::unicode;
use qrcode::QrCode;
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
            match parse_args(&msg.content) {
                Some((_, username)) => {
                    let ((username, platform)) = parse_args(username).unwrap_or(("thiself", "pc"));
                    // TODO implement get_r6_stats
                    let _stats = get_r6_stats(platform, username);
                    if let Err(why) = msg.channel_id.say(&ctx.http, "Coming soon...").await {
                        println!("Error sending message: {:?}", why);
                    }
                }
                None => {
                    if let Err(why) = msg
                    .channel_id
                    .say(
                        &ctx.http,
                        "Make sure you specify the `username` then the `platform` to get stats!",
                    )
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
                }
            }
        }

        if msg.content.starts_with("!qr") {
            if let Some((_, content)) = parse_args(&msg.content) {
                let code = QrCode::new(content.as_bytes()).unwrap();
                let image = code
                    .render::<unicode::Dense1x2>()
                    .dark_color(unicode::Dense1x2::Light)
                    .light_color(unicode::Dense1x2::Dark)
                    .build();
                if let Err(why) = msg
                    .channel_id
                    .say(&ctx.http, format!("Here you go...\n```{}```", image))
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
            } else {
                if let Err(why) = msg
                    .channel_id
                    .say(&ctx.http, "I'll need more than that!")
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
