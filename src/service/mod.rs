use crate::queue::QueueError;
use futures::Stream;
use std::pin::Pin;
use tonic::Status;

pub mod publishing_queue;
impl From<QueueError> for Status {
    fn from(error: QueueError) -> Self {
        let msg = error.to_string();
        match error {
            QueueError::PublishError => Status::internal(msg),
            QueueError::Empty => Status::not_found(msg),
            QueueError::Disconnected => Status::unavailable(msg),
            QueueError::Timeout => Status::deadline_exceeded(msg),
        }
    }
}

type GrpcStream<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send + 'static>>;
