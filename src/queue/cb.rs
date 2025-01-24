use crate::queue::{Queue, QueueConfig, QueueError};
use crossbeam::channel::{RecvTimeoutError, TryRecvError};

pub struct CrossbeamQueue<T> {
    pub queue_config: QueueConfig,
    pub sender: crossbeam::channel::Sender<T>,
    pub receiver: crossbeam::channel::Receiver<T>,
}

impl<T> Queue for CrossbeamQueue<T> {
    type Item = T;

    fn publish_message(&self, item: Self::Item) -> Result<(), QueueError> {
        self.sender.send(item).map_err(|_| QueueError::PublishError)
    }

    fn consume_message(&self) -> Result<Self::Item, QueueError> {
        match self.receiver.try_recv() {
            Ok(item) => Ok(item),
            Err(TryRecvError::Empty) => Err(QueueError::Empty),
            Err(TryRecvError::Disconnected) => Err(QueueError::Disconnected),
        }
    }

    fn consume_message_blocking(&self) -> Result<Self::Item, QueueError> {
        match self.receiver.recv_timeout(self.config().timeout) {
            Ok(item) => Ok(item),
            Err(RecvTimeoutError::Timeout) => Err(QueueError::Timeout),
            Err(RecvTimeoutError::Disconnected) => Err(QueueError::Disconnected),
        }
    }

    fn config(&self) -> &QueueConfig {
        &self.queue_config
    }
}
