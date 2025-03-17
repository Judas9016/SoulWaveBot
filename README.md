# 🤖 Bot de Discord en Rust

Este es un **proyecto personal** desarrollado en Rust utilizando la librería [Serenity](https://github.com/serenity-rs/serenity). Actualmente, el bot incluye comandos básicos como `ping` y `hello`, y en el futuro se expandirá para incluir funcionalidades adicionales, como reproducción de música, administración de servidores y otras herramientas útiles.

## 🚀 Estado del Proyecto

✅ **Comandos Básicos Implementados:**
- 🏓 `ping` – Verifica la latencia del bot.
- 👋 `hello` – Responde con un saludo.

🔜 **Próximas Funciones Planeadas:**
- 🎶 Reproducción de música.
- 🛠️ Comandos de administración.
- 🤖 Interacciones avanzadas con usuarios.

## 🛠️ Instalación y Ejecución

1. **Clona el repositorio:**
   ```bash
   git clone https://github.com/Judas9016/SoulWaveBot.git


2️⃣ Instala Rust y Cargo
Si no tienes Rust instalado, descárgalo desde Rust Lang.

3️⃣ Configura el archivo secrests.toml (debes crearlo manualmente)

Configura el archivo secrets.toml:
Crea manualmente el archivo secrets.toml en la raíz del proyecto y añade al menos la siguiente línea (reemplaza tu_token_aqui por tu token de Discord)

DISCORD_TOKEN=tu_token_aqui

4️⃣ Compila y ejecuta el bot

cargo run --release

📦 Dependencias
Serenity – Para la interacción con Discord.
Tokio – Manejo de asincronía.
Reqwest – Para realizar solicitudes HTTP.
Serde y Serde_json – Para la deserialización de respuestas JSON.
Urlencoding – Para codificar URLs correctamente.
Shuttle-runtime / Shuttle-serenity – Para despliegues (opcional, según configuración).

📜 Notas
Este es un proyecto en desarrollo activo. Se aceptan sugerencias, mejoras y colaboraciones.
El bot está pensado para ser extensible, por lo que en el futuro se incluirán nuevas funcionalidades.

📜 Licencia

Proyecto personal – Uso libre.

---
