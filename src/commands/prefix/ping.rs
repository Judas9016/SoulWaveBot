use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::builder::{CreateEmbed, CreateMessage, EditMessage};
use std::time::Instant;

pub async fn run(ctx: &Context, msg: &Message) {
    let start = Instant::now(); // Guarda el tiempo antes de responder

    // Envía el mensaje inicial con "Calculando..."
    let mut sent_message = match msg.channel_id.send_message(&ctx.http, CreateMessage::new().content("🏓 Calculando...")).await {
        Ok(message) => message,
        Err(why) => {
            println!("Error enviando mensaje: {:?}", why);
            return;
        }
    };

    // Calcula la latencia en milisegundos
    let latency = start.elapsed().as_millis();

    // Crea un embed con la latencia
    let embed = CreateEmbed::new()
        .title("🏓 Pong!")
        .description(format!("📡 **Latencia:** `{}` ms", latency))
        .color(0x3498db);

    // Edita el mensaje original con `EditMessage`
    if let Err(why) = sent_message.edit(&ctx.http, EditMessage::new().embed(embed)).await {
        println!("Error editando mensaje: {:?}", why);
    }
}
