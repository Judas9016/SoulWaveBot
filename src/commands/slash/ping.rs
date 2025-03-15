use serenity::all::*;
use serenity::model::prelude::CommandInteraction;
use std::time::Instant;

pub async fn run(ctx: &Context, command: &CommandInteraction) {
    let start = Instant::now();
    let channel_id = command.channel_id;

    // Responde con "Calculando..." mientras mide la latencia
    let calculating_message = channel_id
        .send_message(&ctx.http, CreateMessage::new().content("⏳ Calculando latencia..."))
        .await;

    if let Err(why) = calculating_message {
        println!("❌ Error enviando mensaje: {:?}", why);
        return;
    }

    // Calcula la latencia en milisegundos
    let latency = start.elapsed().as_millis();

    // Crea un embed para la respuesta
    let embed = CreateEmbed::new()
        .title("🏓 Pong!")
        .description(format!("📡 Latencia: `{}` ms", latency))
        .color(0x5865F2) // Color Discord azul
        .footer(CreateEmbedFooter::new("Tiempo de respuesta medido con precisión."));

    // Edita el mensaje con el resultado final
    if let Err(why) = channel_id
        .send_message(
            &ctx.http,
            CreateMessage::new()
                .content("🏓 ¡Aquí tienes los resultados!")
                .embed(embed),
        )
        .await
    {
        println!("❌ Error editando mensaje: {:?}", why);
    }
}
