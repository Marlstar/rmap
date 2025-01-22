#[derive(Debug, Clone)]
pub enum TcpResult {
    Open{ response: String },
    Closed,
}
