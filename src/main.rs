mod commands;

use std::env::var;

use dotenv;

use serenity::async_trait;
use serenity::model::application::interaction::{ Interaction , InteractionResponseType };
use serenity::model::gateway::Ready;
use serenity::model::prelude::GuildId;
use serenity::prelude::*;

struct Handler {
    database: sqlx::PgPool
}

#[async_trait] 
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            if let Err(why) = match command.data.name.as_str() {
                "ping" => commands::ping::run(&ctx, &command).await,
                "add" => commands::add_suggestion::run(&self.database, &ctx, &command).await,
                "modal" => commands::modal_test::run(&ctx, &command).await,
                _ => {
                    command
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|m| m.content("Not implemented :("))
                        })
                        .await
                }
            }
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
        else if let Interaction::ModalSubmit(command) = interaction {
            println!("Received modal submit interaction: {:#?}", command);

            if let Err(why) = match command.data.custom_id.as_str() {
                "testmodal1" => commands::handle_testmodal1::run(&self.database, &ctx, &command).await,
                _ => {
                    command
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|m| m.content("Not implemented :("))
                        })
                        .await
                }
            }
            {
                println!("Cannot respond to modal: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            var("DISCORD_TEST_GUILD_ID")
                .expect("Missing `DISCORD_TEST_GUILD_ID` env var")
                .parse()
                .expect("DISCORD_TEST_GUILD_ID must be an integer")
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::ping::register(command))
                .create_application_command(|command| commands::add_suggestion::register(command))
                .create_application_command(|command| commands::modal_test::register(command))
        })
        .await;

        println!("I now have the following guild slash commands: {:#?}", commands);
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    
    let token = var("DISCORD_TOKEN").expect("Missing `DISCORD_TOKEN` env var");

    let database = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(var("DATABASE_URL").expect("Missing `DATABASE_URL` env var").as_str())
        .await
        .expect("Couldn't connect to database");

    let bot = Handler {
        database
    };

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(bot)
        .await
        .expect("Error creating client!");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
