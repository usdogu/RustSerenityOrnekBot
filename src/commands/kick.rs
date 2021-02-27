use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        Args, CommandResult,
        macros::command,
    },

};

#[command]
#[only_in(guilds)]
#[required_permissions(KICK_MEMBERS)]
#[max_args(2)]
#[description("Belirtilen Kişiyi Sunucudan Atar")]
#[usage("kick <kullanici> <opsiyonel: sebep>")]
pub async fn kick(ctx: &Context, msg: &Message,mut args: Args) -> CommandResult {
    let user = &msg.mentions[0].id;
    if args.is_empty() {
        let _ = GuildId(msg.guild_id.unwrap().0).kick(&ctx.http, user).await.unwrap();
        let _ = msg.reply(&ctx,format!("{} Kullanıcısı Atıldı",user.mention())).await.unwrap();
    } else {
        let reason = format!("{}", args.single::<String>().unwrap_or_else(|_| "".into()));
        let _ = GuildId(msg.guild_id.unwrap().0).kick_with_reason(&ctx.http, user, reason.as_str()).await.unwrap();
        let _ = msg.reply(&ctx,format!("{} Kullanıcısı {} Sebebiyle Atıldı",user.mention(),reason)).await.unwrap();
    }
    Ok(())
}