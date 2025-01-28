use tomka::configuration::Settings;
use tomka::health_check::HealthService;
use tomka::startup::Application;
use tomka::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("tomka".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let settings = Settings::load().expect("Failed to load configuration");
    let application = Application::build(settings.clone()).await?;
    let application_task = tokio::spawn(application.run());

    let health_check = HealthService::build(settings).await?;
    let health_check_task = tokio::spawn(health_check.run());
    tokio::select! {
        _ = application_task => {},
        _ = health_check_task => {},
    }

    Ok(())
}
