use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum TcpResult {
    Open(Uuid),
    Closed(Uuid),
}
