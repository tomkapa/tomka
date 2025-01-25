use crate::protobuf::queue::publishing_queue_server::PublishingQueue;
use crate::protobuf::queue::{
    ConsumeChunkRequest, ConsumeChunkResponse, Message, PublishChunkRequest, PublishChunkResponse,
};
use crate::queue::{CrossbeamQueue, Queue, QueueError};
use crate::service::GrpcStream;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tonic::{Request, Response, Status};

pub struct PublishingQueueService {
    queue: Arc<CrossbeamQueue<Message>>,
}

impl PublishingQueueService {
    pub fn new(queue: CrossbeamQueue<Message>) -> Self {
        Self {
            queue: Arc::new(queue),
        }
    }
}

#[tonic::async_trait]
impl PublishingQueue for PublishingQueueService {
    async fn publish_chunk(
        &self,
        request: Request<PublishChunkRequest>,
    ) -> Result<Response<PublishChunkResponse>, Status> {
        let req = request.into_inner();
        // check if the chunk is present
        if req.message.is_none() {
            return Err(Status::invalid_argument("Chunk is missing"));
        }
        // Delegate to the common queue logic
        self.queue
            .publish_message(req.message.expect("Chunk must exist"))?;
        Ok(Response::new(PublishChunkResponse {}))
    }

    type ConsumeChunkStream = GrpcStream<ConsumeChunkResponse>;
    async fn consume_chunk(
        &self,
        _request: Request<ConsumeChunkRequest>,
    ) -> Result<Response<Self::ConsumeChunkStream>, Status> {
        let queue = self.queue.clone();
        // Delegate
        let output_stream = async_stream::try_stream! {
            loop {
                match queue.consume_message() {
                    Ok(chunk) => {
                        yield ConsumeChunkResponse{message: Some(chunk)};
                    },
                    Err(QueueError::Empty) => {
                        // no chunk right now, let's sleep briefly or yield
                        // You could do a blocking approach with a channel
                        // listener, or do a short sleep to poll
                        sleep(Duration::from_millis(100)).await;
                    },
                    Err(QueueError::Disconnected) => {
                        // partition queue is closed
                        break;
                    }
                _ => {unreachable!("It should not be possible to get any other error");}
                };
            }
        };
        Ok(Response::new(
            Box::pin(output_stream) as Self::ConsumeChunkStream
        ))
    }
}
