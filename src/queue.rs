use std::sync::Arc;
use crossbeam::queue::SegQueue;
use crate::protobuf::Chunk;

pub struct PublishingQueue {
    // If multi-producer is needed, a thread-safe queue like crossbeam's SegQueue works well.
    chunks: SegQueue<Chunk>,
}

impl PublishingQueue {
    pub fn new() -> Self {
        Self {
            chunks: SegQueue::new(),
        }
    }

    // Multiple or single producers can call this to add chunks
    pub fn push_chunk(&self, chunk: Chunk) {
        self.chunks.push(chunk);
    }

    // Many consumers pop entire chunks
    pub fn pop_chunk(&self) -> Option<Chunk> {
        self.chunks.pop()
    }
}