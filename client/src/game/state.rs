pub enum State {
    Disconnected,
    Connected { GameId: usize },
}
