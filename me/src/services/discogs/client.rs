use crate::services::discogs::types::{Collection, Record, Records};
use reqwest;

// Client interacts with the discogs API using a personal access token. This
// is not meant to be a fully fledged Discogs client: it does NOT go through
// an OAuth flow. This only serves as a quick and dirty way to pull my
// collection for display.
pub struct Client {
    http: reqwest::Client,
    // discogs_url is the endpoint for the discogs API.
    discogs_url: String,
    // username is your discogs username.
    username: String,
    // personal_token is a personal access token used to authorise requests
    // for username.
    personal_token: String,
}

impl Client {
    pub fn new(
        http: reqwest::Client,
        discogs_url: String,
        username: String,
        personal_token: String,
    ) -> Self {
        Self {
            http,
            discogs_url,
            username,
            personal_token,
        }
    }

    // get_collection retrieves count (N) most recent records added to a Discogs
    // folder_id with offset. This is consumed as a route in a Cloudflare
    // worker to show the most recent records I have added to my collection.
    pub async fn get_collection(
        self,
        folder_id: i32,
        offset: i32,
        count: i32,
    ) -> Result<Records, Box<dyn std::error::Error>> {
        let collection_url = format! {
            "{}/users/{}/collection/folders/{}/releases?sort=added&sort_order=desc&page={}&per_page={}",
            self.discogs_url,
            self.username,
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
                format! {"Discogs token={}", self.personal_token},
            )
            .send()
            .await?
            .json::<Collection>()
            .await?;

        Ok(Records(Record::from(resp.clone())))
    }
}
