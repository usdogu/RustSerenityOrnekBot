use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        Args, CommandResult,
        macros::command,
    },
    utils::parse_username,
    
};

#[command]
#[required_permissions("BAN_MEMBERS")]
#[only_in("guild")]
#[max_args(2)]
#[description("Belirtilen Kullanıcının Banını Kaldırır")]
#[usage("unban <kullanici>")]
pub async fn unban(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user1 = &args.single::<String>().unwrap();
    let user = parse_username(user1.as_str()).unwrap();
    let _ = GuildId(msg.guild_id.unwrap().0).unban(&ctx, user).await.unwrap();
    let _ = msg.reply(&ctx,"Kullanıcının Banı Kaldırıldı").await.unwrap();

    

    Ok(())
}