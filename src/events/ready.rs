use serenity::model::prelude::*;

pub async fn en_linea(ready: Ready) {
    println!(
        "\n\x1b[34m######################################\x1b[0m
         \x1b[32m✅🚀 BOT EN LÍNEA\x1b[0m
         \x1b[33m🤖 Nombre:\x1b[0m {}  
         \x1b[36m🕒 Hora de inicio:\x1b[0m {}  
        \x1b[34m######################################\x1b[0m\n",
        ready.user.name,
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S") // Muestra la hora actual
    );
}


#[allow(dead_code)]
pub async fn error_inicio(mensaje: &str) {
    println!(
        "\n\x1b[31m######################################\x1b[0m
         \x1b[91m❌🔥 ERROR AL INICIAR\x1b[0m
         \x1b[33m⚠️ Motivo:\x1b[0m {}  
         \x1b[36m🕒 Hora del error:\x1b[0m {}  
        \x1b[31m######################################\x1b[0m\n",
        mensaje,
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    );
}