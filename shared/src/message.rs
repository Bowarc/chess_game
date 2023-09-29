#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub enum ClientMessage {
    Text(String),
    Ping,
    Pong,
    // Get the games that the server is hosting
    RequestGames,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub enum ServerMessage {
    Text(String),
    Ping,
    Pong,
    // Send a list of games (only send the useful informations, don't give everything)
    Games(Vec<crate::game::Game>),
}

impl networking::Message for ClientMessage {
    fn is_ping(&self) -> bool {
        matches!(self, Self::Ping)
    }
    fn is_pong(&self) -> bool {
        matches!(self, Self::Pong)
    }

    fn default_ping() -> Self {
        Self::Ping
    }
    fn default_pong() -> Self {
        Self::Pong
    }
}

impl networking::Message for ServerMessage {
    fn is_ping(&self) -> bool {
        matches!(self, Self::Ping)
    }
    fn is_pong(&self) -> bool {
        matches!(self, Self::Pong)
    }

    fn default_ping() -> Self {
        Self::Ping
    }
    fn default_pong() -> Self {
        Self::Pong
    }
}
