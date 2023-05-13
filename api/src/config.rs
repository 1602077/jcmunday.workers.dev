#[derive(Debug, Clone)]
pub struct Settings {
    pub discogs: DiscogsSettings,
    pub wakatime: WakatimeSettings,
}

#[derive(Debug, Clone)]
pub struct DiscogsSettings {
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct WakatimeSettings {
    pub url: String,
}

/// get_config returns application level config.
///
/// Compiling to wasm prevents a more complicated management system using yaml
/// files / environment variables as the filesystem is locked out.
pub fn get_config() -> Settings {
    let discogs = DiscogsSettings {
        url: "https://api.discogs.com".to_string(),
    };

    let wakatime = WakatimeSettings {
        url: "https://wakatime.com/api/v1/".to_string(),
    };

    Settings { discogs, wakatime }
}
