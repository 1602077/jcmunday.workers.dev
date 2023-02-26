use crate::services::discogs::client;
use crate::services::discogs::types::CollectionResponse;
use dotenv::dotenv;
use reqwest;

mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let discogs_api_token =
        std::env::var("DISCOGS_API_TOKEN").expect("DISCOGS_API_TOKEN must be set.");
    println!("DISCOGS_API_TOKEN: {:?}", discogs_api_token);

    let discogs_client = client::Client::new(
        reqwest::Client::new(),
        "https://api.discogs.com".to_string(),
        "jcmunday".to_string(),
        discogs_api_token,
    );

    discogs_client.collection(2233333, 1, 3).await?;

    Ok(())
}
