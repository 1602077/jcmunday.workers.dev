use worker::{Context, Env, Request, Router};

use crate::config::Settings;

pub struct Application {
    config: Settings,
    router: worker::Router<()>,
}

impl Application {
    pub async fn build(config: Settings) -> Result<Self, std::io::Error> {
        let router = worker::Router::new();

        let discogs_client = discogs::Client::new(config.discogs);
        let waka_client = waka::Client::new(config.wakatime);

        router
            .get("/", |_, _| Response::ok("hello jack"))
            .get_async("/music", |_req, ctx| async move {
                let my_collection_id = 2233333;
                let start_position = 1;
                let num_records = 10;

                let vinyl = discogs_client
                    .get_collection(my_collection_id, start_position, num_records)
                    .await
                    .expect("expected a response");

                Response::ok(vinyl.to_string().to_owned())
            })
            .get_async("/dev-time", |_, ctx| async move {
                let time = waka_client
                    .get_dev_time()
                    .await
                    .expect("expected a response");

                let dev_time = format! {"Over the {}, I've spent {} coding!",
                    time["data"]["human_readable_range"],
                    time["data"]["human_readable_total"]
                };

                Response::ok(dev_time.to_string().to_owned())
            });

        Self { config, router }
    }

    pub async fn run(self, req: Request, env: Env, _ctx: Context) -> Result<Response> {
        log_request(&req);
        utils::set_panic_hook();

        self.router.run(rew, env).await
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
