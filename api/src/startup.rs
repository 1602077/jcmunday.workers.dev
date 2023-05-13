use worker::*;

use crate::config::Settings;
use crate::routes::dev::get_dev_time;
use crate::routes::health::health;
use crate::routes::music::get_discogs_collection;
use crate::services::discogs::Client as DiscogsClient;
use crate::services::waka::Client as WakaClient;
use crate::utils;

#[derive(Clone, Debug)]
pub struct Application {
    pub discogs_client: DiscogsClient,
    pub waka_client: WakaClient,
}

impl Application {
    pub async fn build(config: Settings) -> Self {
        let discogs_client = DiscogsClient::new(config.discogs);
        let waka_client = WakaClient::new(config.wakatime);

        Self {
            discogs_client,
            waka_client,
        }
    }

    pub async fn run(self, req: Request, env: Env, _ctx: Context) -> Result<Response> {
        log_request(&req);
        utils::set_panic_hook();

        let router = worker::Router::with_data(self);

        router
            .get("/", health)
            .get_async("/music", get_discogs_collection)
            .get_async("/dev-time", get_dev_time)
            .run(req, env)
            .await
    }
}

pub fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}
