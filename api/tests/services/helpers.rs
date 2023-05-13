use std::error::Error;

use api::config::get_config;
use api::services::discogs::{
    Artist, BasicInformation, Collection, CollectionRequest, Records, Release,
};
use api::startup::Application;
use fake::faker::name::en::Name;
use fake::Fake;
use rand::distributions::Alphanumeric;
use rand::Rng;
use secrecy::Secret;
use wiremock::MockServer;

pub struct TestApp {
    pub services: Application,
    pub waka_server: MockServer,
    pub discogs_server: MockServer,
}

impl TestApp {
    pub fn generate_random_token(&self, length: usize) -> Secret<String> {
        Secret::new(
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(length)
                .map(char::from)
                .collect(),
        )
    }

    pub async fn get_discogs_collection(&self) -> Result<Records, Box<dyn Error>> {
        let personal_token = self.generate_random_token(10);
        let username: Secret<String> = Secret::new(Name().fake());
        let collection_id: Secret<String> = Secret::new((1..100).fake::<String>());

        self.services
            .discogs_client
            .clone()
            .get_collection(CollectionRequest {
                username,
                collection_id,
                personal_token,
                offset: 0,
                count: 10,
            })
            .await
    }

    pub fn mock_discogs_collection(&self) -> Collection {
        Collection {
            releases: vec![Release {
                date_added: "July 2020".to_string(),
                basic_information: BasicInformation {
                    title: "What Kinda Music".to_string(),
                    year: 2020,
                    artists: vec![Artist {
                        name: "Tom Misch".to_string(),
                        ..Artist::default()
                    }],
                    ..BasicInformation::default()
                },
                ..Release::default()
            }],
        }
    }
}

pub async fn bootstrap() -> TestApp {
    let waka_server = MockServer::start().await;
    let discogs_server = MockServer::start().await;

    let config = {
        let mut c = get_config();
        c.wakatime.url = waka_server.uri();
        c.discogs.url = discogs_server.uri();
        c
    };

    let services = Application::build(config).await;

    TestApp {
        services,
        waka_server,
        discogs_server,
    }
}
