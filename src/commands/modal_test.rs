use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::component::InputTextStyle;
use serenity::prelude::*;


pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> Result<(), SerenityError> {
  command.create_interaction_response(&ctx.http, |response| {
    response
      .kind(InteractionResponseType::Modal)
      .interaction_response_data(|m| m.content("Hey, I'm still alive!").title("This is for a modal?").custom_id("testmodal1").components(|c| {
        c.create_action_row(|row| {
          row.create_input_text(|input| {
            input.custom_id("input1").label("Input 1").style(InputTextStyle::Short)
          })
        })
      }))
  })
  .await
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
  command.name("modal").description("A command to test modals")
}