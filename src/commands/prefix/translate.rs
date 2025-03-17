use serenity::all::*;
use serenity::Error as SerenityError;
use reqwest::Client;
use serde::Deserialize;
use tokio::time::{sleep, Duration};
use std::sync::Arc;

#[derive(Deserialize, Debug)]
struct MyMemoryResponse {
    responseData: ResponseData,
    responseStatus: i32,
    responseDetails: Option<String>,
}

#[derive(Deserialize, Debug)]
struct ResponseData {
    #[serde(rename = "translatedText")]
    translated_text: String,
    #[serde(rename = "match")]
    match_score: Option<f32>,
}

pub async fn run(ctx: &Context, msg: &Message) {
    // Enviar un mensaje con las opciones de idioma de origen
    let source_msg = msg
    .channel_id
    .send_message(&ctx.http, CreateMessage::new()
        .content("Primero, selecciona el idioma de origen:")
        .embed(CreateEmbed::new()
            .title("Idioma de origen")
            .description("Reacciona con el emoji correspondiente al idioma desde el que deseas traducir:\n\n🇪🇸 Español\n🇬🇧 Inglés\n🇫🇷 Francés\n🇩🇪 Alemán")
        )
    )
    .await
    .expect("Error enviando mensaje");

    // Añadir reacciones para seleccionar el idioma de origen
    let emojis = ["🇪🇸", "🇬🇧", "🇫🇷", "🇩🇪"];
    for emoji in emojis.iter() {
        source_msg
            .react(&ctx.http, ReactionType::Unicode(emoji.to_string()))
            .await
            .expect("Error añadiendo reacción");
    }

    // Esperar a que el usuario reaccione para seleccionar el idioma de origen
    let user_id = msg.author.id;
    let channel_id = msg.channel_id;
    let source_msg_id = source_msg.id;

    let ctx = Arc::new(ctx.clone());
    let ctx_clone = Arc::clone(&ctx);
    let source_handle = tokio::spawn(async move {
        // Esperar hasta 30 segundos para una reacción
        for _ in 0..30 {
            // Obtener las reacciones del mensaje
            if let Ok(message) = channel_id.message(&ctx_clone.http, source_msg_id).await {
                for reaction in message.reactions {
                    if let Ok(users) = channel_id.reaction_users(&ctx_clone.http, source_msg_id, reaction.reaction_type.clone(), None, None).await {
                        if users.iter().any(|u| u.id == user_id) {
                            let lang = match reaction.reaction_type.to_string().as_str() {
                                "🇪🇸" => "es",
                                "🇬🇧" => "en",
                                "🇫🇷" => "fr",
                                "🇩🇪" => "de",
                                _ => {
                                    return None;
                                }
                            };
                            return Some(lang.to_string());
                        }
                    }
                }
            }

            // Esperar 1 segundo antes de volver a verificar
            sleep(Duration::from_secs(1)).await;
        }

        None
    });

    // Obtener el idioma de origen seleccionado con un timeout explícito
    let source_lang = match tokio::time::timeout(Duration::from_secs(30), source_handle).await {
        Ok(result) => match result {
            Ok(lang) => lang,
            Err(_) => None,
        },
        Err(_) => None, // Timeout alcanzado
    };

    if let Some(source_lang) = source_lang {
        // Ahora solicitar el idioma de destino
        let target_msg = msg
        .channel_id
        .send_message(&ctx.http, CreateMessage::new()
            .content(format!("Idioma de origen seleccionado: **{}**. Ahora, selecciona el idioma de destino:", get_language_name(&source_lang)))
            .embed(CreateEmbed::new()
                .title("Idioma de destino")
                .description("Reacciona con el emoji correspondiente al idioma al que deseas traducir:\n\n🇪🇸 Español\n🇬🇧 Inglés\n🇫🇷 Francés\n🇩🇪 Alemán")
            )
        )
        .await
        .expect("Error enviando mensaje");

        // Añadir reacciones para seleccionar el idioma de destino
        for emoji in emojis.iter() {
            target_msg
                .react(&ctx.http, ReactionType::Unicode(emoji.to_string()))
                .await
                .expect("Error añadiendo reacción");
        }

        // Esperar a que el usuario reaccione para seleccionar el idioma de destino
        let target_msg_id = target_msg.id;
        let ctx_clone = Arc::clone(&ctx);
        let target_handle = tokio::spawn(async move {
            // Esperar hasta 30 segundos para una reacción
            for _ in 0..30 {
                // Obtener las reacciones del mensaje
                if let Ok(message) = channel_id.message(&ctx_clone.http, target_msg_id).await {
                    for reaction in message.reactions {
                        if let Ok(users) = channel_id.reaction_users(&ctx_clone.http, target_msg_id, reaction.reaction_type.clone(), None, None).await {
                            if users.iter().any(|u| u.id == user_id) {
                                let lang = match reaction.reaction_type.to_string().as_str() {
                                    "🇪🇸" => "es",
                                    "🇬🇧" => "en",
                                    "🇫🇷" => "fr",
                                    "🇩🇪" => "de",
                                    _ => {
                                        return None;
                                    }
                                };
                                return Some(lang.to_string());
                            }
                        }
                    }
                }

                // Esperar 1 segundo antes de volver a verificar
                sleep(Duration::from_secs(1)).await;
            }

            None
        });

        // Obtener el idioma de destino seleccionado con un timeout explícito
        let target_lang = match tokio::time::timeout(Duration::from_secs(30), target_handle).await {
            Ok(result) => match result {
                Ok(lang) => lang,
                Err(_) => None,
            },
            Err(_) => None, // Timeout alcanzado
        };

        if let Some(target_lang) = target_lang {
            // Verificar que el idioma de origen y destino sean diferentes
            if source_lang == target_lang {
                let _ = enviar_respuesta(&ctx, &msg, "❌ Los idiomas de origen y destino son iguales. Por favor, intenta de nuevo con idiomas diferentes.").await;
                return;
            }
            
            // Pedir el texto a traducir
            let prompt_msg = enviar_respuesta(&ctx, &msg, &format!(
                "Traducción de **{}** a **{}**. Por favor, escribe el texto que deseas traducir:",
                get_language_name(&source_lang),
                get_language_name(&target_lang)
            )).await;
            
            // Guardar el ID del mensaje de solicitud para compararlo después
            let prompt_msg_id = match prompt_msg {
                Ok(msg) => msg.id,
                Err(_) => msg.id, // Fallback en caso de error
            };

            // Esperar a que el usuario escriba el texto con un timeout explícito
            let text_future = async {
                let mut text_msg: Option<Message> = None;
                
                for _ in 0..30 {
                    // Esperar 1 segundo para dar tiempo al usuario para escribir
                    sleep(Duration::from_secs(1)).await;
                    
                    // Obtener los mensajes más recientes
                    if let Ok(messages) = msg.channel_id.messages(&ctx.http, GetMessages::new().limit(10)).await {
                        // Buscar un mensaje del autor original que sea más reciente que nuestro mensaje de solicitud
                        for message in messages {
                            if message.author.id == msg.author.id && message.id > prompt_msg_id {
                                return Some(message);
                            }
                        }
                    }
                }
                
                None
            };
            
            let text_msg = match tokio::time::timeout(Duration::from_secs(30), text_future).await {
                Ok(result) => result,
                Err(_) => None, // Timeout alcanzado
            };

            if let Some(text_msg) = text_msg {
                let text = text_msg.content.trim();
                
                // Traducir el texto con los idiomas seleccionados
                match translate_text(text, &source_lang, &target_lang).await {
                    Ok(translated) => {
                        let source_name = get_language_name(&source_lang);
                        let target_name = get_language_name(&target_lang);
                        let response = format!("🔠 **Traducción ({} → {})**\n💬 `{}`\n➡️ `{}`", source_name, target_name, text, translated);
                        let _ = enviar_respuesta(&ctx, &msg, &response).await;
                    },
                    Err(e) => {
                        let error_msg = format!("❌ Error en la traducción: {:?}", e);
                        let _ = enviar_respuesta(&ctx, &msg, &error_msg).await;
                    }
                }
            } else {
                let _ = enviar_respuesta(&ctx, &msg, "❌ Tiempo de espera agotado. No se recibió ningún texto.").await;
            }
        } else {
            let _ = enviar_respuesta(&ctx, &msg, "❌ Tiempo de espera agotado. No se seleccionó el idioma de destino.").await;
        }
    } else {
        let _ = enviar_respuesta(&ctx, &msg, "❌ Tiempo de espera agotado. No se seleccionó el idioma de origen.").await;
    }
}

// Función para obtener el nombre del idioma a partir del código
fn get_language_name(code: &str) -> &str {
    match code {
        "es" => "Español",
        "en" => "Inglés",
        "fr" => "Francés",
        "de" => "Alemán",
        _ => "Desconocido",
    }
}

async fn translate_text(text: &str, source_lang: &str, target_lang: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let url = format!(
        "https://api.mymemory.translated.net/get?q={}&langpair={}|{}&de=your_email@example.com",
        text, source_lang, target_lang
    );

    let res = client.get(&url).send().await?;
    let body: MyMemoryResponse = res.json().await?;
    Ok(body.responseData.translated_text)
}

async fn enviar_respuesta(ctx: &Context, msg: &Message, content: &str) -> Result<Message, SerenityError> {
    msg.channel_id.say(&ctx.http, content).await
}