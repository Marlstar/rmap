use crate::Task;

pub enum WorkerMessage {
    Done
}

pub enum ManagerMessage {
    Task(Task),
    Exit
}
