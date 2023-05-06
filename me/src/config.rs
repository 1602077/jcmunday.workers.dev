use secrecy::Secret;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Settings {
    pub discogs: DiscogSettings,
    pub wakatime: WakatimeSettings,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct DiscogSettings {
    pub url: &str,
    pub username: Secret<String>,
    pub collection_id: Secret<String>,
    pub personal_token: Secret<String>,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct WakatimeSettings {
    pub url: &str,
    pub personal_token: Secret<String>,
}

pub fn get_config(ctx: worker::Context) -> Result<Settings, config::ConfigError> {
    let discogs_username: Secret<String> = Secret::new(ctx.secret("DISCOGS_USERNAME")?.to_string());
    let discogs_personal_token: Secret<String> =
        Secret::new(ctx.secret("DISCOGS_API_TOKEN")?.to_string());

    let discogs = DiscogSettings {
        url: "https://api.discogs.com",
        username: discogs_username,
        collection_id: None,
        personal_token: discogs_personal_token,
    };

    let waka_personal_token: Secret<String> =
        Secret::new(ctx.secret("WAKATIME_API_TOKEN")?.to_string());

    let wakatime = WakatimeSettings {
        url: "https://wakatime.com/api/v1/",
        personal_token: waka_personal_token,
    };

    Ok(Settings { discogs, wakatime })
}
