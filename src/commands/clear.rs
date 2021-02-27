use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        Args, CommandResult,
        macros::command,
    },
    //futures::StreamExt,
};

#[command]
#[only_in("guild")]
#[max_args(1)]
#[min_args(1)]
#[required_permissions("MANAGE_MESSAGES")]
#[description("Belirtilen Sayı Kadar Mesajı Siler")]
#[usage("clear <sayı>")]
pub async fn clear(ctx: &Context, msg: &Message,mut args: Args) -> CommandResult {
    
    let messages = ChannelId(msg.channel_id.0).messages(&ctx.http,|m| {
        
        let limit = args.single::<u64>().unwrap();
        m.limit(limit + 1);
        m

    }).await.unwrap();
    
    for i in messages {
        i.delete(&ctx).await.unwrap();
    }
    

    Ok(())
}