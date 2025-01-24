use crate::protobuf::queue::publishing_queue_server::PublishingQueue;
use crate::protobuf::queue::{
    ConsumeChunkRequest, ConsumeChunkResponse, Message, PublishChunkRequest, PublishChunkResponse,
};
use crate::queue::{CrossbeamQueue, Queue};
use tonic::{Request, Response, Status};

pub struct PublishingQueueService {
    queue: CrossbeamQueue<Message>,
}

impl PublishingQueueService {
    pub fn new(queue: CrossbeamQueue<Message>) -> Self {
        Self { queue }
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
        if req.chunk.is_none() {
            return Err(Status::invalid_argument("Chunk is missing"));
        }
        // Delegate to the common queue logic
        self.queue
            .publish_message(req.chunk.expect("Chunk must exist"))
            .map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(PublishChunkResponse {}))
    }

    async fn consume_chunk(
        &self,
        _request: Request<ConsumeChunkRequest>,
    ) -> Result<Response<ConsumeChunkResponse>, Status> {
        // Delegate
        let result = self
            .queue
            .consume_message()
            .map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(ConsumeChunkResponse {
            chunk: Some(result),
        }))
    }
}
