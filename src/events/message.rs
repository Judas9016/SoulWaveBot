use serenity::prelude::*;
use serenity::model::prelude::*;
use crate::commands::prefix; // Importa comandos con prefijo

pub async fn on_message(ctx: &Context, msg: &Message) {
    if msg.author.bot {
        return;
    }

    let content = msg.content.to_lowercase();

    match content.as_str() {
        "!ping" => prefix::ping::run(ctx, msg).await,
        "!hello" => prefix::hello::hello(ctx, msg).await,
        _ => {}
    }
}
