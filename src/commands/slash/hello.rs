use serenity::all::*;
use serenity::model::prelude::CommandInteraction;
use std::time::Instant;

pub async fn run(ctx: &Context, command: &CommandInteraction) {
    let start = Instant::now();
    let channel_id = command.channel_id;

    // Responde con "Escribiendo..." mientras procesa el saludo
    let calculating_message = channel_id
        .send_message(&ctx.http, CreateMessage::new().content("⌨️ Escribiendo..."))
        .await;

    if let Err(why) = calculating_message {
        println!("❌ Error enviando mensaje: {:?}", why);
        return;
    }

    // Calcula el tiempo de respuesta
    let response_time = start.elapsed().as_millis();

    // Crea un embed para la respuesta
    let embed = CreateEmbed::new()
        .title("👋 ¡Saludos!")
        .description(format!("Hola, {}! Espero que estés teniendo un gran día. ☀️", command.user.name))
        .color(0x00FF00) // Verde
        .footer(CreateEmbedFooter::new(format!("Tiempo de respuesta: {} ms", response_time)));

    // Edita el mensaje con el resultado final
    if let Err(why) = channel_id
        .send_message(
            &ctx.http,
            CreateMessage::new()
                .content("💬 ¡Aquí tienes tu saludo!")
                .embed(embed),
        )
        .await
    {
        println!("❌ Error enviando mensaje: {:?}", why);
    }
}
