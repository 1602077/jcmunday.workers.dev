use claims::{assert_err, assert_ok};
use wiremock::matchers::any;
use wiremock::{Mock, ResponseTemplate};

use crate::helpers::bootstrap;

#[tokio::test]
async fn test_get_dev_time_does_not_error_on_200() {
    let app = bootstrap().await;

    Mock::given(any())
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            r#"{"time_hrs": "8"}"#.as_bytes().to_owned(),
            "application/json",
        ))
        .mount(&app.waka_server)
        .await;

    let personal_token = app.generate_random_token(10);

    let resp = app.services.waka_client.get_dev_time(personal_token).await;

    assert_ok!(resp);
}

#[tokio::test]
async fn test_get_dev_time_errors_if_server_returns_a_500() {
    let app = bootstrap().await;

    Mock::given(any())
        .respond_with(ResponseTemplate::new(500))
        .mount(&app.waka_server)
        .await;

    let personal_token = app.generate_random_token(10);

    let resp = app.services.waka_client.get_dev_time(personal_token).await;

    assert_err!(resp);
}
