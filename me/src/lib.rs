mod config;
mod services;
mod startup;
mod utils;

use worker::{Context, Env, Request, Result};

use crate::{config::get_config, startup::Application};

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let config = get_config(_ctx)?;
    Application::build(config).await?.run(req, env, ctx)
}
