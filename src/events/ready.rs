use serenity::all::*;
use chrono::Local;

pub async fn en_linea(ctx: &Context, ready: Ready) {
    println!(
        "\n\x1b[34m######################################\x1b[0m
         \x1b[32m✅🚀 BOT EN LÍNEA\x1b[0m
         \x1b[33m🤖 Nombre:\x1b[0m {}  
         \x1b[36m🕒 Hora de inicio:\x1b[0m {}  
        \x1b[34m######################################\x1b[0m\n",
        ready.user.name,
        Local::now().format("%Y-%m-%d %H:%M:%S") // Muestra la hora actual
    );

    // 🔹 Registra los slash commands cuando el bot inicia
    if let Err(why) = registrar_comandos(ctx).await {
        println!("❌ Error registrando comandos: {:?}", why);
    }
}

pub async fn registrar_comandos(ctx: &Context) -> Result<(), serenity::Error> {
    let commands = vec![
        CreateCommand::new("ping").description("Comprueba la latencia del bot"),
        CreateCommand::new("hello").description("Recibe un saludo amistoso"),
        CreateCommand::new("translate")
            .description("Traduce texto a otro idioma")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "idioma",
                    "Idioma de destino",
                )
                .required(true),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "texto",
                    "Texto a traducir",
                )
                .required(true),
            ),
    ];

    Command::set_global_commands(&ctx.http, commands).await?;
    println!("✅ Slash commands registrados correctamente.");
    Ok(())
}

