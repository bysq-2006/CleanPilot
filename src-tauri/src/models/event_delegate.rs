use std::sync::{Arc, Mutex};

use tokio::sync::mpsc;

#[derive(Debug)]
pub struct EventDelegate {
    pub sender: mpsc::Sender<String>,
    pub receiver: Arc<Mutex<Option<mpsc::Receiver<String>>>>,
}

impl EventDelegate {
    pub fn new(buffer: usize) -> Self {
        let (sender, receiver) = mpsc::channel::<String>(buffer);

        Self {
            sender,
            receiver: Arc::new(Mutex::new(Some(receiver))),
        }
    }
}
