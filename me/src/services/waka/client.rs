const WAKATIME_API: &str = "https://wakatime.com/api/v1/";

// Client interacts with the wakatime API. Once again I have not bothered with
// OAuth and am instead using a personal access token.
pub struct Client {
    http: reqwest::Client,
    // api_token is the base64 encoded wakatime secret api.
    api_token: String,
}

impl Client {
    pub fn new(http: reqwest::Client, api_token: String) -> Self {
        Self { http, api_token }
    }
    // get_dev_time uses the wakatime API to return stats on development
    // activities over the last 7 days.
    pub async fn get_dev_time(self) -> Result<json::JsonValue, Box<dyn std::error::Error>> {
        let stats_url = format!("{}/users/current/stats/last_7_days", WAKATIME_API);
        let resp = self
            .http
            .get(stats_url)
            .header("Authorization", format! {"Basic {}",self.api_token})
            .send()
            .await?
            .text()
            .await?;

        Ok(json::parse(&resp).unwrap())
    }
}
