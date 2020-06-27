use std::borrow::Cow;

use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};
use serenity::http::AttachmentType;

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.say(&ctx.http, "Pong!");

    Ok(())
}

static WEEABOO: &[u8] = include_bytes!("../../static/weeaboo.png");

#[command]
fn weeaboo(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.content("I think I just heard someone say \"Weeaboo\"");
        m.add_file(AttachmentType::Bytes{ data: Cow::Borrowed(WEEABOO), filename: "weeaboo.png".to_string()});
        m
    });

    Ok(())
}