use crate::CollectionResponse;
use reqwest;

pub struct Client {
    http: reqwest::Client,
    discogs_url: String,
    username: String,
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
    pub async fn collection(
        self,
        folder_id: i32,
        offset: i32,
        count: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
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
                format! {"Discogs token={}",self.personal_token},
            )
            .send()
            .await?;

        // println!("{:#?}", resp.text().await?);

        let body = &resp.json::<CollectionResponse>().await?;
        println!("Body:\n{:#?}", body);

        // TODO: Parse to custom type & return

        Ok(())
    }
}
