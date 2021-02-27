mod commands;

use std::{
    collections::HashSet,
    sync::Arc
};
use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::{
        StandardFramework,
        standard::macros::group,
                
    },
    http::Http,
    model::{
        gateway::Ready,
    },
    prelude::*
};
use commands::{
    ban::*,
    unban::*,
    clear::*,
    kick::*,
    komut::*,
};
#[group]
#[commands(ban,unban,kick,clear,komut_calistir)]
struct General;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        println!("Hazır {}",_data_about_bot.user.name);
    }
    
}


#[tokio::main]
async fn main() {
    
    let http = Http::new_with_token("bot tokeni buraya");
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Uygulama bilgisine erişilemedi: {:?}", why),
    };
    let framework = StandardFramework::new()
    .configure(|c| 
    c.owners(owners)
    .prefix("/"))
    .group(&GENERAL_GROUP);
    let mut client = Client::builder("bot tokeni buraya")
    .framework(framework)
    .event_handler(Handler).await.unwrap();
    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }
    let shardmanager = client.shard_manager.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        shardmanager.lock().await.shutdown_all().await;
    });
    client.start().await.unwrap()
}
