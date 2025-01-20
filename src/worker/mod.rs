mod tcp;
pub use tcp::TcpChecker;

mod tcp_result;
pub use tcp_result::TcpResult;
// ================
use crate::{WorkerMessage, ManagerMessage, Task, TaskResult};
use std::sync::mpsc;

pub struct Worker {
    comms_tx: mpsc::SyncSender<WorkerMessage>,
    comms_rx: mpsc::Receiver<ManagerMessage>,
    results_tx: mpsc::SyncSender<TaskResult>
}
impl Worker { // Constructors
    pub fn new(results_tx: mpsc::SyncSender<TaskResult>) -> (Self, mpsc::SyncSender<ManagerMessage>, mpsc::Receiver<WorkerMessage>) {
        let (manager_comms_tx, comms_rx) = mpsc::sync_channel::<ManagerMessage>(3);
        let (comms_tx, manager_comms_rx) = mpsc::sync_channel::<WorkerMessage>(3);

        let worker = Worker {
            comms_tx,
            comms_rx,
            results_tx
        };

        return (worker, manager_comms_tx, manager_comms_rx);
    }
}
impl Worker {
    pub fn main(&self) {
        'main: loop {
            let msg = match self.comms_rx.try_recv() {
                Ok(a) => a,
                Err(e) => match e {
                    mpsc::TryRecvError::Empty => {
                        continue 'main;
                    },
                    mpsc::TryRecvError::Disconnected => {
                        panic!("manager sender died!")
                    }
                }
            };

            match msg {
                ManagerMessage::Task(task) => {
                    let _ = self.results_tx.send(Self::run_task(task));
                },
            }
        }
    }

    pub fn run_task(task: Task) -> TaskResult {
        let uuid = task.uuid;
        let runner = crate::worker::TcpChecker::new(task);
        return TaskResult {
            result: runner.check(),
            uuid
        };
    }
}
