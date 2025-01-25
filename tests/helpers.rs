use std::net::SocketAddr;
use std::sync::LazyLock;
use tomka::telemetry::{get_subscriber, init_subscriber};

// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: LazyLock<()> = LazyLock::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

pub struct TestApp {
    pub address: SocketAddr,
}

impl TestApp {
    pub async fn new() -> Self {
        // Ensure that the `tracing` stack is only initialised once
        LazyLock::force(&TRACING);

        let settings = {
            let mut s =
                tomka::configuration::Settings::load().expect("Failed to load configuration");
            s.application.grpc_port = 0;
            s
        };
        let application = tomka::startup::Application::build(settings)
            .await
            .expect("Failed to build application");
        let application_address = application.address();
        let _ = tokio::spawn(application.run());

        Self {
            address: application_address,
        }
    }
}
