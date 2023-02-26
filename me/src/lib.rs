use crate::services::discogs::client;
use worker::event;
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

    let router = Router::new();

    router
        .get("/", |_, _| Response::ok("hello jack!"))
        .get_async("/music", |_, ctx| async move {
            let discogs_username = ctx.secret("DISCOGS_USERNAME")?.to_string();
            let discogs_api_token = ctx.secret("DISCOGS_API_TOKEN")?.to_string();

            let discogs_client = client::Client::new(
                reqwest::Client::new(),
                "https://api.discogs.com".to_string(),
                discogs_username.to_string(),
                discogs_api_token,
            );

            let vinyl = discogs_client
                .get_collection(2233333, 1, 5)
                .await
                .expect("expected a response");

            Response::ok(vinyl.to_string().to_owned())
        })
        .get("/worker-version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();

            Response::ok(version)
        })
        .run(req, env)
        .await
}
