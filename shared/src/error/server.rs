#[derive(thiserror::Error, Debug)]
pub enum ServerError{
    #[error(transparent)]
    Game(GameError),
    #[error(transparent)]
    Client(ClientError)
}

#[derive(thiserror::Error, Debug)]
pub enum GameError{
    #[error("The session is full")]
    SessionIsFull
}

#[derive(thiserror::Error, Debug)]
pub enum ClientError{
    #[error("The proxy for the client '{0}' has disconnected")]
    ProxyDisconnected(std::net::SocketAddr)
}