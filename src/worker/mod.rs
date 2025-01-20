mod tcp;
pub use tcp::TcpChecker;

mod tcp_result;
pub use tcp_result::TcpResult;
// ================
use crate::{ManagerMessage, Task, TaskResult, WorkerMessage};
use std::sync::mpsc;

#[derive(Debug)]
pub struct Worker {
    comms_tx: crate::WorkerTX,
    comms_rx: crate::WorkerRX,
    results_tx: crate::ResultsTX
}
impl Worker { // Constructors
    pub fn new(results_tx: crate::ResultsTX) -> (Self, crate::ManagerTX, crate::ManagerRX) {
        let (manager_comms_tx, comms_rx) = mpsc::sync_channel(3);
        let (comms_tx, manager_comms_rx) = mpsc::sync_channel(3);

        let worker = Worker {
            comms_tx,
            comms_rx,
            results_tx
        };

        return (worker, manager_comms_tx, manager_comms_rx);
    }
}
impl Worker {
    pub fn run(&self) {
        'main: loop {
            let msg = match self.comms_rx.try_recv() {
                Ok(a) => a,
                Err(e) => match e {
                    mpsc::TryRecvError::Empty => {
                        continue 'main;
                    },
                    mpsc::TryRecvError::Disconnected => {
                        eprintln!("Manager sender died, worker exiting!");
                        return;
                    }
                }
            };

            match msg {
                ManagerMessage::Task(task) => {
                    let _ = self.results_tx.send(Self::run_task(task));
                    let _ = self.comms_tx.send(WorkerMessage::Done);
                },
                ManagerMessage::Exit => {
                    break 'main;
                }
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
