use std::fmt::Write as _;

use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::component::ButtonStyle;
use serenity::prelude::*;


pub async fn run(db: &sqlx::PgPool, ctx: &Context, command: &ApplicationCommandInteraction) -> Result<(), SerenityError> {
  let suggestions = sqlx::query!("SELECT value FROM suggestions")
    .fetch_all(db)
    .await
    .unwrap();

  let mut content = format!("There are {} suggestions!\n", suggestions.len());

  for (i, suggestion) in suggestions.iter().enumerate() {
    writeln!(content, "{}. {}", i + 1, suggestion.value).unwrap();
  }

  command.create_interaction_response(&ctx.http, |response| {
    response
      .kind(InteractionResponseType::ChannelMessageWithSource)
      .interaction_response_data(|m| {
        m
          .content(content)
          .components(|components| {
            components.create_action_row(|row| {
              row.create_button(|button| {
                button
                  .custom_id("tribename.addsuggestion")
                  .label("Make a Suggestion")
                  .style(ButtonStyle::Success)
              })
            })
          })
    })
  })
  .await
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
  command.name("list-suggestions").description("List tribe name suggestions")
}