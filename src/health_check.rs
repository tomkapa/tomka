use crate::configuration::Settings;
use axum::{routing::get, Router};
use tokio::net::TcpListener;

async fn health_check_handler() -> &'static str {
    "healthy"
}

pub struct HealthService {
    port: u16,
    server: axum::serve::Serve<TcpListener, Router, Router>,
}

impl HealthService {
    pub async fn build(settings: Settings) -> anyhow::Result<Self> {
        let address = format!(
            "{}:{}",
            settings.application.host, settings.application.http_port
        );
        let listener = TcpListener::bind(address).await?;
        let addr = listener.local_addr()?;

        let app = Router::new().route("/", get(health_check_handler));

        // Build the server
        let server = axum::serve(listener, app);
        Ok(HealthService {
            port: addr.port(),
            server,
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run(self) -> anyhow::Result<()> {
        tracing::info!("Health check server running on port {}", self.port);
        self.server.await?;
        Ok(())
    }
}
