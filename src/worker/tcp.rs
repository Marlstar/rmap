use std::io::Read;
use std::net::TcpStream;
use std::time::Duration;
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

        let mut tcp = match connection_result {
            // In future this could pass the tcp stream off to do some analysis
            Ok(a) => a,
            Err(_) => return TcpResult::Closed
        };

        std::thread::sleep(Duration::from_secs(1));
        let mut buf = String::new();

        'read: {
            if tcp.set_read_timeout(Some(Duration::from_secs(1))).is_err() {break 'read;};
            if tcp.read_to_string(&mut buf).is_err() { break 'read }
        }

        let _ = tcp.shutdown(std::net::Shutdown::Both);
        return TcpResult::Open{
            response: buf
        };
    }
}

