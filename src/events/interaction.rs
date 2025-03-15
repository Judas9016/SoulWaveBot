use serenity::all::*;
use crate::commands::slash; // Importa comandos slash

pub async fn handle_interaction(ctx: &Context, interaction: Interaction) {
    if let Interaction::Command(ref command) = interaction {
        match command.data.name.as_str() {
            "ping" => slash::ping::run(ctx, command).await,
            "hello" => slash::hello::run(ctx, command).await,
            _ => println!("Comando no reconocido"),
        }
    }
}
