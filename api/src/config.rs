// TODO (jack): config management using a configuration file for non secrets.
use secrecy::Secret;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Settings {
    pub discogs: DiscogSettings,
    pub wakatime: WakatimeSettings,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct DiscogSettings {
    pub url: String,
    pub username: Secret<String>,
    // pub collection_id: Secret<String>,
    pub personal_token: Secret<String>,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct WakatimeSettings {
    pub url: String,
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    // pub fn get_config(ctx: worker::RouteContext<Context>) -> Result<Settings, config::ConfigError> {
    // let discogs_username: Secret<String> = Secret::new(
    //     ctx.secret("DISCOGS_USERNAME")
    //         .expect("failed to get discogs username")
    //         .to_string(),
    // );
    // let discogs_personal_token: Secret<String> = Secret::new(
    //     ctx.secret("DISCOGS_API_TOKEN")
    //         .expect("failed to get discogs api token")
    //         .to_string(),
    // );

    let discogs = DiscogSettings {
        url: "https://api.discogs.com".to_string(),
        username: Secret::new("xxx".to_string()),
        // username: discogs_username,
        // collection_id: None,
        personal_token: Secret::new("xxx".to_string()),
        // personal_token: discogs_personal_token,
    };

    // let waka_personal_token: Secret<String> = Secret::new(
    //     ctx.secret("WAKATIME_API_TOKEN")
    //         .expect("failed to get wakatime api token")
    //         .to_string(),
    // );

    let wakatime = WakatimeSettings {
        url: "https://wakatime.com/api/v1/".to_string(),
    };

    Ok(Settings { discogs, wakatime })
}
