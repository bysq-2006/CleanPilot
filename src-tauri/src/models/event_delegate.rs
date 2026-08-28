use std::sync::{Arc, Mutex};

use tokio::sync::{mpsc, oneshot};

pub struct EventDelegateRequest {
    pub payload: String,
    pub reply: oneshot::Sender<Result<(), String>>,
}

#[derive(Debug, Clone)]
pub struct EventDelegate {
    pub sender: mpsc::Sender<EventDelegateRequest>,
    pub receiver: Arc<Mutex<Option<mpsc::Receiver<EventDelegateRequest>>>>,
}

impl EventDelegate {
    pub fn new(buffer: usize) -> Self {
        let (sender, receiver) = mpsc::channel::<EventDelegateRequest>(buffer);

        Self {
            sender,
            receiver: Arc::new(Mutex::new(Some(receiver))),
        }
    }

    /// 发送业务事件并等待 Manager 处理结果。失败会回到调用方，便于写入工具上下文。
    pub async fn request(&self, payload: String) -> Result<(), String> {
        let (reply, rx) = oneshot::channel();
        self.sender
            .send(EventDelegateRequest { payload, reply })
            .await
            .map_err(|_| "事件委托通道已关闭".to_string())?;
        rx.await
            .map_err(|_| "事件委托处理结果丢失".to_string())?
    }
}
