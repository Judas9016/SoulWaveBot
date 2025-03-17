use serenity::all::*;
use reqwest::Client;
use serde::{Deserialize};

#[derive(Deserialize)]
struct MyMemoryResponse {
    responseData: ResponseData,
}

#[derive(Deserialize)]
struct ResponseData {
    translatedText: String,
}

pub async fn run(ctx: &Context, command: &CommandInteraction) {
    let options = &command.data.options;

    let lang = options.iter().find(|o| o.name == "idioma").and_then(|o| o.value.as_str());
    let text = options.iter().find(|o| o.name == "texto").and_then(|o| o.value.as_str());

    if lang.is_none() || text.is_none() {
        enviar_respuesta(ctx, command, "❌ Uso: /translate idioma:es texto:Hello world").await;
        return;
    }

    let lang = lang.unwrap();
    let text = text.unwrap();

    match translate_text(text, lang).await {
        Ok(translated) => {
            let response = format!("🔠 **Traducción ({})**\n💬 `{}`\n➡️ `{}`", lang, text, translated);
            enviar_respuesta(ctx, command, &response).await;
        }
        Err(e) => {
            let error_msg = format!("❌ Error en la traducción: {:?}", e);
            enviar_respuesta(ctx, command, &error_msg).await;
        }
    }
}

async fn translate_text(text: &str, target_lang: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let url = format!(
        "https://api.mymemory.translated.net/get?q={}&langpair=en|{}",
        text, target_lang
    );

    let res = client.get(&url).send().await?;
    let body: MyMemoryResponse = res.json().await?;
    
    Ok(body.responseData.translatedText)
}

async fn enviar_respuesta(ctx: &Context, command: &CommandInteraction, content: &str) {
    let message = CreateInteractionResponseMessage::new().content(content);
    let response = CreateInteractionResponse::Message(message);

    if let Err(why) = command.create_response(&ctx.http, response).await {
        println!("❌ Error enviando respuesta: {:?}", why);
    }
}
