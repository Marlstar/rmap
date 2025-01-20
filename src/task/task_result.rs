use crate::worker::TcpResult;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TaskResult {
    pub result: TcpResult,
    pub uuid: Uuid
}
