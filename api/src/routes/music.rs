use secrecy::Secret;

use crate::services::discogs::CollectionRequest;
use crate::startup::Application;

pub async fn get_discogs_collection(
    _req: worker::Request,
    ctx: worker::RouteContext<Application>,
) -> Result<worker::Response, worker::Error> {
    let username: Secret<String> = Secret::new(
        ctx.secret("DISCOGS_USERNAME")
            .expect("failed to get wakatime api token")
            .to_string(),
    );

    let collection_id: Secret<String> = Secret::new(
        ctx.secret("DISCOGS_COLLECTION_ID")
            .expect("failed to get wakatime api token")
            .to_string(),
    );

    let personal_token: Secret<String> = Secret::new(
        ctx.secret("DISCOGS_API_TOKEN")
            .expect("failed to get wakatime api token")
            .to_string(),
    );

    let request = CollectionRequest {
        username,
        collection_id,
        personal_token,
        // TODO: These will eventually be populated by request body from frontend.
        offset: 1,
        count: 10,
    };

    let vinyl = ctx
        .data
        .discogs_client
        .get_collection(request)
        .await
        .expect("expected a respose");

    worker::Response::ok(vinyl.to_string())
}
