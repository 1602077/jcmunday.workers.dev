use api::config::get_config;
use api::startup::Application;
use rand::distributions::Alphanumeric;
use rand::Rng;
use secrecy::Secret;
use wiremock::MockServer;

pub struct TestApp {
    pub services: Application,
    pub waka_server: MockServer,
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
}

pub async fn bootstrap() -> TestApp {
    let waka_server = MockServer::start().await;

    let config = {
        let mut c = get_config();
        c.wakatime.url = waka_server.uri();
        c
    };

    let services = Application::build(config).await;

    TestApp {
        services,
        waka_server,
    }
}
