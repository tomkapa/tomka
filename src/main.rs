use tomka::configuration::Settings;
use tomka::startup::Application;
use tomka::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("tomka".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let settings = Settings::load().expect("Failed to load configuration");
    let application = Application::build(settings).await?;
    let application_task = tokio::spawn(application.run());
    tokio::select! {
        _ = application_task => {},
    }

    Ok(())
}
