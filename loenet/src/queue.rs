use crossbeam_queue::SegQueue;
use serde_json::Value;
use std::sync::{Arc, Mutex};
use crate::message::Message;

#[derive(Debug)]
pub struct OutboundQueue {
    queue: Arc<Mutex<SegQueue<Message>>>,
}

impl OutboundQueue {
    pub fn new() -> Self {
        OutboundQueue {
            queue: Arc::new(Mutex::new(SegQueue::new())),
        }
    }

    pub fn push(&self, message: Message) {
        self.queue.lock().unwrap().push(message);
    }

    pub fn pop(&self) -> Option<Message> {
        self.queue.lock().unwrap().pop()
    }
}

#[derive(Debug)]
pub struct InboundQueue {
    // NOTE: This is why it supports only one app per machine; No in-machine routing
    queue: Arc<Mutex<SegQueue<Value>>>,
}

impl InboundQueue {
    pub fn new() -> Self {
        InboundQueue {
            queue: Arc::new(Mutex::new(SegQueue::new())),
        }
    }

    pub fn push(&self, message: Value) {
        self.queue.lock().unwrap().push(message);
    }

    pub fn pop(&self) -> Option<Value> {
        self.queue.lock().unwrap().pop()
    }
}
