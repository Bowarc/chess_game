#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Game {
    pub id: crate::id::Id,
    pub players: [Option<Player>; 2],
    pub state: State,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Player {
    pub id: crate::id::Id,
    pub name: String,
    pub color: Option<crate::chess::Color>,
}

#[derive(
    Default,
    Debug,
    Clone,
    enum_variant_name::VariantName,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum State {
    PlayerDisconnected,
    #[default]
    Waiting,
    GameStart,
    Playing {
        // infos about the games / board etc..
        board: crate::chess::Board,
    },
    GameEnd {
        winner: Option<crate::id::Id>,
    },
}

impl Game {
    pub fn new(id: crate::id::Id, players: [Option<Player>; 2], state: State) -> Self {
        Self { id, players, state }
    }

    pub fn id(&self) -> crate::id::Id {
        self.id
    }

    pub fn player_count(&self) -> u64 {
        self.players
            .iter()
            .filter(|&player| player.is_some())
            .count() as u64
    }
    pub fn max_players(&self) -> u64 {
        2
    }

    pub fn players(&self) -> &[Option<Player>; 2] {
        &self.players
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut State {
        &mut self.state
    }
}

impl Player {
    pub fn new(id: crate::id::Id, name: String, color: Option<crate::chess::Color>) -> Self {
        Self { id, name, color }
    }
}
