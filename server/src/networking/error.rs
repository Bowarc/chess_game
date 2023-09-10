#[derive(thiserror::Error, Debug)]
pub enum NetworkError {
    #[error("{0}")]
    Socket(#[from] shared::networking::SocketError),
    #[error("{0}")]
    ChannelSend(String),
}
