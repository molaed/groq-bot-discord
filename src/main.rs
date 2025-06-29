use std::env::var;

use groq_api_rust::{AsyncGroqClient, ChatCompletionMessage, ChatCompletionRoles, ChatCompletionRequest};
use once_cell::sync::Lazy;

use serenity::{
    async_trait,
    model::channel::Message,
    prelude::*
};

static DISCORD_TOK_LAZY: Lazy<String> = Lazy::new(|| {
    dotenv::dotenv().expect("No .env file!");
    var("DISCORD_API_TOKEN").expect("Discord token must be set!")
});

static GROQ_TOK_LAZY: Lazy<String> = Lazy::new(|| {
    dotenv::dotenv().expect("No .env file!");
    var("GROQ_API_TOKEN").expect("Groq token must be set!")
});

static GROQ_SYSTEM_PROMPT: &str = r#"

"#;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
     async fn message(&slf, _ctx: Context, msg: Message) {

     }
}

#[tokio::main]
async fn main() {
    println!("Starting up Groq Bot...");

    // Initialize groq client 
    let groq_token = Lazy::force(&GROQ_TOK_LAZY);
    let groq_client = AsyncGroqClient::new(groq_token.clone(), None).await;

    let messages = vec![
        ChatCompletionMessage {
            role: ChatCompletionRoles::System,
            content: GROQ_SYSTEM_PROMPT.to_string(), 
            name: None,
        },
    ];
    let request = ChatCompletionRequest::new("llama3-70b-8192", messages);
    
    if cfg!(not(debug_assertions)) {
        let response = tokio::join!(groq_client.chat_completion(request)).0;
        let response = response.expect("Failed to get response for request");
        println!("{}", response.choices[0].message.content);
        assert!(!response.choices.is_empty()); 
    }

    // Initialize discord client 
    let discord_token = Lazy::force(&DISCORD_TOK_LAZY);
    let intents =
        GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut discord_client = Client::builder(discord_token.clone(), intents)
        .event_handler(Handler)
        .await
        .expect("Error creating discord client");

    if let Err(why) = client.start().await {
        printlin!("Client error: {why:?}");
    }

}
