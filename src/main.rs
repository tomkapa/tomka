use crossbeam::channel::unbounded;
use tomka::protobuf::queue::publishing_queue_server::PublishingQueueServer;
use tomka::protobuf::queue::result_queue_server::ResultQueueServer;
use tomka::queue::CrossbeamQueue;
use tomka::service::publishing_queue::PublishingQueueService;
use tomka::service::result_queue::ResultQueueService;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // -------------------------------
    // 1) Create Crossbeam channels
    // SPMC (single producer, multiple consumers) for the publishing queue
    // In crossbeam, the channel is actually MPMC, but we will only use
    // one "sender" for the publisher side to keep it effectively SPMC.
    let (pub_s, pub_r) = unbounded();
    let publishing_queue = CrossbeamQueue {
        queue_config: Default::default(),
        sender: pub_s,
        receiver: pub_r,
    };

    // MPSC (multiple producers, single consumer) for result queue
    // Again, crossbeam is MPMC, but we only use one "receiver" for the aggregator
    let (res_s, res_r) = unbounded();
    let result_queue = CrossbeamQueue {
        queue_config: Default::default(),
        sender: res_s,
        receiver: res_r,
    };

    // -------------------------------
    // 2) Build service implementations
    let publishing_service = PublishingQueueService::new(publishing_queue);
    let result_service = ResultQueueService::new(result_queue);

    // -------------------------------
    // 3) Run gRPC server
    let addr = "127.0.0.1:50051".parse()?;
    println!("Starting queue service on {}", addr);

    Server::builder()
        .add_service(PublishingQueueServer::new(publishing_service))
        .add_service(ResultQueueServer::new(result_service))
        .serve(addr)
        .await?;

    Ok(())
}
