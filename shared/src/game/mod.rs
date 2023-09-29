#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub struct Game {
    id: crate::id::Id,
    player1: Option<Player>,
    player2: Option<Player>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub struct Player {
    id: crate::id::Id,
    name: String,
}

impl Game {
    pub fn new(id: crate::id::Id, player1: Option<Player>, player2: Option<Player>) -> Self {
        Self {
            id,
            player1,
            player2,
        }
    }
}

impl Player {
    pub fn new(id: crate::id::Id, name: String) -> Self {
        Self { id, name }
    }
}
