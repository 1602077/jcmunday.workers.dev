use claims::{assert_err, assert_ok};
use wiremock::matchers::any;
use wiremock::{Mock, ResponseTemplate};

use crate::helpers::bootstrap;

#[tokio::test]
async fn test_get_collection_does_not_error_on_200() {
    let app = bootstrap().await;
    // let sample_response = Records(vec![Record {
    //     album: "What Kinda Music".to_string(),
    //     artist: "Tom Misch".to_string(),
    //     date_added: "July 2020".to_string(),
    //     pressed: "July 2020".to_string(),
    // }]);
    let sample_response = app.mock_discogs_collection();

    Mock::given(any())
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response))
        .mount(&app.discogs_server)
        .await;

    let resp = app.get_discogs_collection().await;

    assert_ok!(resp);
}

#[tokio::test]
async fn test_get_collection_errors_if_server_returns_a_500() {
    let app = bootstrap().await;

    Mock::given(any())
        .respond_with(ResponseTemplate::new(500))
        .mount(&app.discogs_server)
        .await;

    let resp = app.get_discogs_collection().await;

    assert_err!(resp);
}
