use secrecy::Secret;

use crate::startup::Application;

pub async fn get_dev_time(
    _req: worker::Request,
    ctx: worker::RouteContext<Application>,
) -> Result<worker::Response, worker::Error> {
    let personal_token: Secret<String> = Secret::new(
        ctx.secret("WAKATIME_API_TOKEN")
            .expect("failed to get wakatime api token")
            .to_string(),
    );

    let time = ctx
        .data
        .waka_client
        .get_dev_time(personal_token)
        .await
        .expect("failed to get dev time");

    let dev_time = format! {"Over the {}, I've spent {} coding!",
        time["data"]["human_readable_range"],
        time["data"]["human_readable_total"]
    };

    worker::Response::ok(dev_time)
}
