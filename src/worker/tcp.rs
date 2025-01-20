use std::net::TcpStream;
use crate::worker::TcpResult;
use crate::Task;

pub struct TcpChecker {
    task: Task
}
impl TcpChecker { // Constructors
    pub fn new(task: Task) -> Self {
        Self { task }
    }
}
impl TcpChecker {
    pub fn check(&self) -> TcpResult {
        let addr = self.task.addr;
        let connection_result = TcpStream::connect(addr);

        let tcp = match connection_result {
            // In future this could pass the tcp stream off to do some analysis
            Ok(a) => a,
            Err(_) => return TcpResult::Closed
        };

        let _ = tcp.shutdown(std::net::Shutdown::Both);
        return TcpResult::Open;
    }
}

