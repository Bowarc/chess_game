#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub struct Game {
    id: crate::id::Id,
    players: [Option<Player>; 2],
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub struct Player {
    id: crate::id::Id,
    name: String,
}

impl Game {
    pub fn new(id: crate::id::Id, players: [Option<Player>; 2],) -> Self {
        Self {
            id,
            players
        }
    }

    pub fn id(&self) -> crate::id::Id{
        self.id
    }

    pub fn player_count(&self) -> usize{
        self.players.iter().filter(|&player| player.is_some()).count()
    }
    pub fn max_players(&self) -> usize{
        2
    }
}

impl Player {
    pub fn new(id: crate::id::Id, name: String) -> Self {
        Self { id, name }
    }
}
