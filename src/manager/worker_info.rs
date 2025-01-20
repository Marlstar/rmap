use uuid::Uuid;
use crate::{ManagerMessage, Task, WorkerMessage};

#[derive(Debug)]
pub struct WorkerInfo {
    pub handle: std::thread::JoinHandle<()>,
    pub id: Uuid,
    pub tx: crate::ManagerTX,
    pub rx: crate::ManagerRX,
}
impl WorkerInfo {
    pub fn send_task(&self, task: Task) {
        // TODO: error handling
        let _ = self.tx.try_send(ManagerMessage::Task(task));
    }

    pub fn send_message(&self, message: ManagerMessage) {
        // TODO: error handling
        let _ = self.tx.try_send(message);
    }

    pub fn receive_messages(&self) -> Vec<WorkerMessage> {
        let mut messages: Vec<WorkerMessage> = vec![];
        while let Ok(msg) = self.rx.try_recv() {
            messages.push(msg);
        }
        return messages;
    }
}
