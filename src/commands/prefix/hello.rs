use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn hello(ctx: &Context, msg: &Message) {
    let response = format!("¡Hola, {}! 👋", msg.author.name);
    if let Err(e) = msg.channel_id.say(&ctx.http, response).await {
        println!("Error enviando mensaje: {:?}", e);
    }
}
