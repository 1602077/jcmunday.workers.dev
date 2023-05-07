use reqwest;
use secrecy::ExposeSecret;

use crate::{
    config::DiscogSettings,
    services::discogs::types::{Collection, Record, Records},
};

// Client interacts with the discogs API using a personal access token. This
// is not meant to be a fully fledged Discogs client: it does NOT go through
// an OAuth flow. This only serves as a quick and dirty way to pull my
// collection for display.
#[derive(Clone)]
pub struct Client {
    http: reqwest::Client,
    config: DiscogSettings,
}

impl Client {
    pub fn new(config: DiscogSettings) -> Self {
        // todo!("connection pool")
        let http = reqwest::Client::new();

        Self {
            http,
            config: config.clone(),
        }
    }

    // get_collection retrieves count (N) most recent records added to a Discogs
    // folder_id with offset. This is consumed as a route in a Cloudflare
    // worker to show the most recent records I have added to my collection.
    pub async fn get_collection(
        self,
        folder_id: &i32,
        offset: &i32,
        count: &i32,
    ) -> Result<Records, Box<dyn std::error::Error>> {
        let collection_url = format! {
            "{}/users/{}/collection/folders/{}/releases?sort=added&sort_order=desc&page={}&per_page={}",
            self.config.url,
            self.config.username.expose_secret(),
            folder_id.to_string(),
            offset.to_string(),
            count.to_string(),
        };

        let resp = self
            .http
            .get(collection_url)
            .header(reqwest::header::USER_AGENT, "curl/7.84.0")
            .header(
                "Authorization",
                format! {"Discogs token={}", self.config.personal_token.expose_secret()},
            )
            .send()
            .await?
            .json::<Collection>()
            .await?;

        Ok(Records(Record::from(resp.clone())))
    }
}
