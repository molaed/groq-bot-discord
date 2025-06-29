use std::env::var;

use dotenv::dotenv;
use once_cell::sync::Lazy;

static DISCORD_TOKEN: Lazy<String> = Lazy::new(|| {
    dotenv::dotenv().expect("No .env file!");
    var("DISCORD_API_TOKEN").expect("Discord token must be set!")
});

static GROQ_TOKEN: Lazy<String> = Lazy::new(|| {
    dotenv::dotenv().expect("No .env file!");
    var("GROQ_API_TOKEN").expect("Groq token must be set!")
});


fn main() {
        println!("Hello, world!");
}
