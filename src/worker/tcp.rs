use std::net::{Ipv4Addr, SocketAddrV4};
use std::net::TcpStream;
use super::TcpResult;

pub struct TcpChecker {
    addr: SocketAddrV4
}
impl TcpChecker { // Constructors
    pub fn new(ip: Ipv4Addr, port: u16) -> Self {
        Self {
            addr: SocketAddrV4::new(ip, port)
        }
    }
}
impl TcpChecker {
    pub fn check(&self) -> TcpResult {
        let connection_result = TcpStream::connect(self.addr);

        let tcp = match connection_result {
            // In future this could pass the tcp stream off to do some analysis
            Ok(a) => a,
            Err(_) => return TcpResult::Closed
        };

        let _ = tcp.shutdown(std::net::Shutdown::Both);
        return TcpResult::Open;
    }
}

