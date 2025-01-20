#![allow(dead_code, clippy::needless_return)]

pub mod worker;
pub use worker::Worker;

mod task;
pub use task::{Task, TaskResult};

pub mod manager;

pub mod message;
pub use message::WorkerMessage;
pub use message::ManagerMessage;

mod args;
pub use args::Args;
