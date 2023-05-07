mod config;
mod services;
mod startup;
mod utils;

use worker::{Context, Env, Request, Response, Result, Router};

use crate::{
    config::get_config,
    startup::{log_request, Application},
};

#[worker::event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    log_request(&req);
    utils::set_panic_hook();

    let configuration = get_config().expect("failed to get config");

    let app = Application::build(configuration).await;

    let router = Router::with_data(app);
    router
        .get("/", |_, _| Response::ok("hello jack"))
        .run(req, env)
        .await
}
