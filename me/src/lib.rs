use crate::services::discogs::client as discogs;
use crate::services::waka::client as waka;
use worker::*;

mod services;
mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);
    utils::set_panic_hook();
    let router = worker::Router::new();

    router
        .get("/", |_, _| Response::ok("hello jack"))
        .get_async("/music", |_req, ctx| async move {
            let discogs_username = ctx.secret("DISCOGS_USERNAME")?.to_string();
            let discogs_api_token = ctx.secret("DISCOGS_API_TOKEN")?.to_string();

            let discogs_client = discogs::Client::new(
                reqwest::Client::new(),
                "https://api.discogs.com".to_string(),
                discogs_username.to_string(),
                discogs_api_token,
            );

            let vinyl = discogs_client
                .get_collection(2233333, 1, 10)
                .await
                .expect("expected a response");

            Response::ok(vinyl.to_string().to_owned())
        })
        .get_async("/dev-time", |_, ctx| async move {
            let waka_api_token = ctx.secret("WAKATIME_API_TOKEN")?.to_string();
            let waka_client = waka::Client::new(reqwest::Client::new(), waka_api_token);
            let time = waka_client
                .get_dev_time()
                .await
                .expect("expected a response");

            let dev_time = format! {"Over the {}, I've spent {} coding!",
                time["data"]["human_readable_range"],
                time["data"]["human_readable_total"]
            };

            Response::ok(dev_time.to_string().to_owned())
        })
        .run(req, env)
        .await
}
