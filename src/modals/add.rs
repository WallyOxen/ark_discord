use serenity::model::prelude::InteractionResponseType;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::component::{InputTextStyle, ActionRowComponent};
use serenity::model::prelude::modal::ModalSubmitInteraction;
use serenity::prelude::*;

use uuid::Uuid;

pub async fn create(command: &ApplicationCommandInteraction, ctx: &Context) -> Result<(), SerenityError> {
  command.create_interaction_response(&ctx.http, |response| {
    response
      .kind(InteractionResponseType::Modal)
      .interaction_response_data(|m| m.title("Make a tribe name suggestion").custom_id("tribename.addsuggestion").components(|c| {
        c.create_action_row(|row| {
          row.create_input_text(|input| {
            input.custom_id("suggestion").label("Suggestion").style(InputTextStyle::Short)
          })
        })
      }))
  })
  .await
}

pub async fn run(db: &sqlx::PgPool, ctx: &Context, command: &ModalSubmitInteraction) -> Result<(), SerenityError> {
  let inputs: Vec<String> = command
    .data
    .components
    .iter()
    .filter_map(|row| match row.components.first() {
      Some(ActionRowComponent::InputText(text)) => Some(text.value.clone()),
      Some(_) => None,
      None => None
  }).collect();

  let suggestion = &inputs[0].trim().to_lowercase();

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
    .await
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