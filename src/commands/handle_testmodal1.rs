use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::application::interaction::modal::ModalSubmitInteraction;
use serenity::model::prelude::component::ActionRowComponent;
use serenity::prelude::*;
use uuid::Uuid;

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