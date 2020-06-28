use serenity::model::gateway::Activity;
use serenity::model::channel::Message;
use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::client::Context;
use serenity::model::user::OnlineStatus;

#[command]
#[owners_only]
fn listen(ctx: &mut Context, _: &Message, mut args: Args) -> CommandResult {
    let activity_name = args.single::<String>()?;

    let activity = Activity::listening(&activity_name);
    let status = OnlineStatus::Online;

    ctx.set_presence(Some(activity), status);

    Ok(())
}