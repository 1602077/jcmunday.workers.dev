use worker::*;

use crate::{
    config::Settings,
    routes::{dev::get_dev_time, health::health},
    services::{discogs::Client as DiscogsClient, waka::Client as WakaClient},
    utils,
};

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
            .get_async("/dev-time", get_dev_time)
            // .get_async("/music", |_req, ctx| async move {
            //     let token = ctx
            //         .secret("DISCOGS_API_TOKEN")
            //         .expect("failed to get wakatime api token")
            //         .to_string();
            //     dbg!(token);
            //     Response::ok("aaa".to_owned())
            // })
            // .get_async("/music", |_req, ctx| async move {
            //     let my_collection_id = 2233333;
            //     let start_position = 1;
            //     let num_records = 10;
            //     // let vv = self.discogs_client.get_collection(0, 0, 0).await;
            //     let vinyl = self
            //         .discogs_client
            //         .get_collection(&2233333, &1, &10)
            //         .await
            //         .expect("aaa");
            //     Response::ok(vinyl.to_string().to_owned())
            // })
            // .get_async("/dev-time", |_req, _ctx| async move {
            // let time = self
            //     .waka_client
            //     .get_dev_time()
            //     .await
            //     .expect("expected a response");
            // let dev_time = format! {"Over the {}, I've spent {} coding!",
            //     time["data"]["human_readable_range"],
            //     time["data"]["human_readable_total"]
            // };
            // Response::ok(dev_time.to_string().to_owned())
            // })
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
