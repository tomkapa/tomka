// src/publishing_queue_service.rs
use std::time::Duration;

mod cb;

pub use cb::CrossbeamQueue;

#[derive(thiserror::Error, Debug)]
pub enum QueueError {
    #[error("Failed to publish message")]
    PublishError,
    #[error("Queue is empty")]
    Empty,
    #[error("Queue disconnected")]
    Disconnected,
    #[error("Failed to consume message due to timeout")]
    Timeout,
}

#[derive(Debug)]
pub struct QueueConfig {
    pub timeout: Duration,
}

impl Default for QueueConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(5),
        }
    }
}

pub trait Queue {
    type Item;
    fn publish_message(&self, item: Self::Item) -> Result<(), QueueError>;
    fn consume_message(&self) -> Result<Self::Item, QueueError>;
    fn consume_message_blocking(&self) -> Result<Self::Item, QueueError>;
    fn config(&self) -> &QueueConfig;
}
