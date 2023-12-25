#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub enum ClientMessage {
    Text(String),
    Ping,
    Pong,
    // Get the games that the server is hosting
    MyIdRequest,

    RequestGames,
    GameJoinRequest(super::id::Id),
    GameInfoRequest(super::id::Id),
    GameCreateRequest,
    LeaveGameRequest,

    // Gaming time
    MakeMove(super::chess::ChessMove),
    
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub enum ServerMessage {
    Text(String),
    Ping,
    Pong,
    // Send a list of games (only send the useful informations, don't give everything)
    PlayerIdResponse(crate::id::Id),

    Games(Vec<crate::game::Game>),
    GameJoin(super::game::Game),
    GameLeave,
    GameJoinFaill(String),
    GameInfoUpdate(crate::id::Id, crate::game::Game),
    GameInfoUpdateFail(crate::id::Id, String),
    GameCreateSucess(crate::id::Id),
    GameCreatefail(String),

    // Game time
    MoveResponse{
        chess_move: super::chess::ChessMove,
        valid: bool,
    }

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
