use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        Args, CommandResult,
        macros::command,
    },

};

#[command]
#[required_permissions("BAN_MEMBERS")]
#[only_in("guild")]
#[max_args(2)]
#[description("Belirtilen Kullanıcıyı Sunucudan Banlar")]
#[usage("ban <kullanici> <opsiyonel: sebep>")]
pub async fn ban(ctx: &Context, msg: &Message,mut args: Args) -> CommandResult {
    let user = &msg.mentions[0];
    if args.is_empty() {
        let _ = GuildId(msg.guild_id.unwrap().0).ban(&ctx.http, user, 0).await.unwrap();
        let _ = msg.reply(&ctx,format!("{} Kullanıcısı Banlandı",user.mention()));
    } else {
        let reason = format!("{}", &args.single::<String>().unwrap_or_else(|_| "".into()));
        let _ = GuildId(msg.guild_id.unwrap().0).ban_with_reason(&ctx.http, user, 0, &reason).await.unwrap();
        let _ = msg.reply(&ctx,format!("{} Kullanıcısı {} Sebebiyle Banlandı",user.mention(),reason)).await.unwrap();
    }
    
    

    Ok(())
}