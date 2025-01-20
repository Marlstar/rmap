#![allow(dead_code, clippy::needless_return)]

pub mod worker;
pub use worker::Worker;
pub type WorkerRX = std::sync::mpsc::Receiver<ManagerMessage>;
pub type WorkerTX = std::sync::mpsc::SyncSender<WorkerMessage>;

pub mod manager;
pub use manager::Manager;
pub type ManagerRX = std::sync::mpsc::Receiver<WorkerMessage>;
pub type ManagerTX = std::sync::mpsc::SyncSender<ManagerMessage>;
pub type ResultsRX = std::sync::mpsc::Receiver<TaskResult>;
pub type ResultsTX = std::sync::mpsc::SyncSender<TaskResult>;

mod task;
pub use task::{Task, TaskResult};

pub mod message;
pub use message::WorkerMessage;
pub use message::ManagerMessage;

mod args;
pub use args::Args;
