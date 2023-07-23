use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::prelude::*;


pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> Result<(), SerenityError> {
  command.create_interaction_response(&ctx.http, |response| {
    response
      .kind(InteractionResponseType::ChannelMessageWithSource)
      .interaction_response_data(|m| m.content("Hey, I'm still alive!"))
  })
  .await
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
  command.name("ping").description("A ping command")
}