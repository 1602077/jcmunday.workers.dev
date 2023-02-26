mod services;
use dotenv::dotenv;
use reqwest;
// use services::discogs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load discogs api key
    dotenv().ok();
    let discogs_api_token =
        std::env::var("DISCOGS_API_TOKEN").expect("DISCOGS_API_TOKEN must be set.");
    println!("DISCOGS_API_TOKEN: {:?}", discogs_api_token);

    let client = reqwest::Client::new();
    let  resp = client
        .get("https://api.discogs.com/users/jcmunday/collection/folders/2233333/releases?sort=added&sort_order=desc&page=1&per_page=1")
        .header(reqwest::header::USER_AGENT, "curl/7.84.0")
        .header("Authorization", format!{"Discogs token={discogs_api_token}"})
        .send().await?;

    println!("Status: {}", resp.status());

    let body = &resp.json::<serde_json::Value>().await?;
    println!("Body:\n{:#?}", body);

    Ok(())
}
