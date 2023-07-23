use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::command::CommandOptionType;
use serenity::prelude::*;

use uuid::Uuid;


pub async fn run(db: &sqlx::PgPool, ctx: &Context, command: &ApplicationCommandInteraction) -> Result<(), SerenityError> {
  let options = &command.data.options;

  let suggestion = options.get(0).expect("Expected string option").value.as_ref().unwrap().as_str().unwrap().to_lowercase();

  let existing = sqlx::query!("SELECT value FROM suggestions WHERE value = $1", suggestion)
    .fetch_all(db)
    .await
    .unwrap();

  if existing.len() > 0 {
    return command.create_interaction_response(&ctx.http, |response| {
      response
        .kind(InteractionResponseType::ChannelMessageWithSource)
        .interaction_response_data(|m| m.content(format!("{} has already been suggested!", &suggestion)).ephemeral(true))
    })
    .await;
  }
  
  sqlx::query!("INSERT INTO suggestions (id, value, userid, username) VALUES ($1, $2, $3, $4)",
    Uuid::new_v4(),
    suggestion,
    command.user.id.to_string(),
    command.user.name
  )
  .execute(db)
  .await
  .unwrap();

  command.create_interaction_response(&ctx.http, |response| {
    response
      .kind(InteractionResponseType::ChannelMessageWithSource)
      .interaction_response_data(|m| m.content(format!("Successfully added {}", &suggestion)).ephemeral(true))
  })
  .await
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
  command.name("add")
    .description("Add a new tribe name suggestion")
    .create_option(|o| {
      o.name("suggestion")
        .description("The name you want to suggest")
        .kind(CommandOptionType::String)
        .required(true)
    })
}