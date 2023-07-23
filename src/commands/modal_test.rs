use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;

use crate::modals;


pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> Result<(), SerenityError> {
  modals::add::create(command, ctx).await
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
  command.name("modal").description("A command to test modals")
}