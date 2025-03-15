use anyhow::Context as _;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::Interaction;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;

mod commands;
mod events;


/// Define la estructura del bot
struct Bot;

#[async_trait]
impl EventHandler for Bot {
    /// Evento cuando el bot se conecta
    #[allow(unused_variables)]
    async fn ready(&self, ctx: Context, ready: Ready) {
        events::ready::en_linea(ready).await;
    }

    /// Evento cuando recibe un mensaje
    async fn message(&self, ctx: Context, msg: Message) {
        events::message::on_message(&ctx, &msg).await;
    }

    /// Evento cuando se ejecuta un slash command
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        events::interaction::handle_interaction(&ctx, interaction).await;
    }
}



//inicia el bot
#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = secrets
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}


