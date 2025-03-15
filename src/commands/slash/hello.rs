use serenity::all::*;

use serenity::model::prelude::CommandInteraction;


pub async fn run(ctx: &Context, command: &CommandInteraction) {
    let response = format!("¡Hola, {}! 👋", command.user.name);

    let data = CreateInteractionResponseMessage::new().content(response);
    let builder = CreateInteractionResponse::Message(data);

    if let Err(why) = command.create_response(&ctx.http, builder).await {
        println!("Error enviando respuesta: {:?}", why);
    }
}
