use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        Args, CommandResult,
        macros::command,
        
    },

};
use std::process::Command;
#[command]
#[required_permissions("ADMINISTRATOR")]
#[description("Botun Çalıştığı Sistemde Komut Çalıştırır")]
#[usage("komut_calistir <komutlar>")]
#[aliases("shell","term")]
pub async fn komut_calistir(ctx: &Context, msg:&Message, args: Args) -> CommandResult {
    let output = Command::new("bash").arg("-c").arg(args.message()).output().unwrap();
    msg.reply(&ctx,String::from_utf8_lossy(&output.stdout)).await.unwrap();
    Ok(())
}