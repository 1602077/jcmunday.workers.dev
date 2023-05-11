use reqwest;
use secrecy::{ExposeSecret, Secret};

use crate::{
    config::DiscogsSettings,
    services::discogs::domain::{Collection, Record, Records},
};

// Client interacts with the discogs API using a personal access token. This
// is not meant to be a fully fledged Discogs client: it does NOT go through
// an OAuth flow. This only serves as a quick and dirty way to pull my
// collection for display.
#[derive(Clone, Debug)]
pub struct Client {
    http: reqwest::Client,
    config: DiscogsSettings,
}

impl Client {
    pub fn new(config: DiscogsSettings) -> Self {
        // TODO: connection pool
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
        request: CollectionRequest,
    ) -> Result<Records, Box<dyn std::error::Error>> {
        // TODO: This feels not great cloning straight away, but I am yet to
        // learn how to use lifetimes properly.
        let collection_url = request.clone().collection_url(self.config.url);

        let resp = self
            .http
            .get(collection_url.expose_secret())
            .header(reqwest::header::USER_AGENT, "curl/7.84.0")
            .header(
                "Authorization",
                format! {
                    "Discogs token={}",
                    request.personal_token.expose_secret().clone()
                },
            )
            .send()
            .await?
            // TODO: error handling on server error.
            .json::<Collection>()
            .await?;

        Ok(Records(Record::from(resp.clone())))
    }
}

#[derive(Debug, Clone)]
pub struct CollectionRequest {
    pub username: Secret<String>,
    pub collection_id: Secret<String>,
    pub personal_token: Secret<String>,
    pub offset: i32,
    pub count: i32,
}

impl CollectionRequest {
    pub fn collection_url(self, base_url: String) -> Secret<String> {
        Secret::new(format! {
            "{}/users/{}/collection/folders/{}/releases?sort=added&sort_order=desc&page={}&per_page={}",
            base_url,
            self.username.expose_secret(),
            self.collection_id.expose_secret(),
            self.offset.to_string(),
            self.count.to_string(),
        })
    }
}
