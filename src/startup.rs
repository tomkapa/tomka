use crate::configuration::Settings;
use crate::protobuf::queue::publishing_queue_server::PublishingQueueServer;
use crate::queue::CrossbeamQueue;
use crate::service::publishing_queue::PublishingQueueService;
use crossbeam::channel::unbounded;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tonic::codegen::tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::server::Router;
use tonic::transport::Server;

pub struct Application {
    address: SocketAddr,
    router: Router,
    tcp_listener_stream: TcpListenerStream,
}

impl Application {
    pub async fn build(settings: Settings) -> anyhow::Result<Self> {
        let address = format!(
            "{}:{}",
            settings.application.host, settings.application.grpc_port
        );
        let listener = TcpListener::bind(address).await?;
        let addr = listener.local_addr()?;
        let incoming = TcpListenerStream::new(listener);

        let (pub_s, pub_r) = unbounded();
        let publishing_queue = CrossbeamQueue {
            queue_config: Default::default(),
            sender: pub_s,
            receiver: pub_r,
        };
        let publishing_service = PublishingQueueService::new(publishing_queue);
        let router = Server::builder().add_service(PublishingQueueServer::new(publishing_service));
        Ok(Self {
            address: addr,
            router,
            tcp_listener_stream: incoming,
        })
    }

    pub fn address(&self) -> SocketAddr {
        self.address
    }

    pub async fn run(self) -> anyhow::Result<()> {
        tracing::info!("Server running on {}", self.address);
        self.router
            .serve_with_incoming(self.tcp_listener_stream)
            .await?;
        Ok(())
    }
}
