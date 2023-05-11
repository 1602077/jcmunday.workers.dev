// TODO (jack): config management using a configuration file for non secrets.

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Settings {
    pub discogs: DiscogsSettings,
    pub wakatime: WakatimeSettings,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct DiscogsSettings {
    pub url: String,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct WakatimeSettings {
    pub url: String,
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    let discogs = DiscogsSettings {
        url: "https://api.discogs.com".to_string(),
    };

    let wakatime = WakatimeSettings {
        url: "https://wakatime.com/api/v1/".to_string(),
    };

    Ok(Settings { discogs, wakatime })
}
