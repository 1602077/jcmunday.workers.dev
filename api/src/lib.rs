mod config;
mod routes;
mod services;
mod startup;
mod utils;

use worker::{Context, Env, Request, Response, Result};

use crate::{
    config::get_config,
    startup::{log_request, Application},
};

#[worker::event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    log_request(&req);
    utils::set_panic_hook();

    let configuration = get_config();

    let app = Application::build(configuration).await;
    app.run(req, env, _ctx).await
}
