use secrecy::{ExposeSecret, Secret};

use crate::config::WakatimeSettings;

// Client interacts with the wakatime API. Once again I have not bothered with
// OAuth and am instead using a personal access token.
pub struct Client {
    http: reqwest::Client,
    config: WakatimeSettings,
}

impl Client {
    pub fn new(config: WakatimeSettings) -> Self {
        let http = reqwest::Client::new();

        Self { http, config }
    }

    // get_dev_time returns stats on development activities over the last 7 days.
    pub async fn get_dev_time(self) -> Result<json::JsonValue, Box<dyn std::error::Error>> {
        let stats_url = format!("{}/users/current/stats/last_7_days", self.config.url);
        let resp = self
            .http
            .get(stats_url)
            .header(
                "Authorization",
                format! {"Basic {}",self.config.personal_token.expose_secret()},
            )
            .send()
            .await?
            .text()
            .await?;

        // TODO: a bit lazy here
        Ok(json::parse(&resp).unwrap())
    }
}
