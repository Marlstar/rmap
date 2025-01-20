mod task_result;
pub use task_result::TaskResult;
//==========
use std::net::SocketAddrV4;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Task {
    pub addr: SocketAddrV4,
    pub uuid: Uuid
}
