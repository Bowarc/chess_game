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

    pub fn id(&self) -> crate::id::Id{
        self.id
    }

    pub fn player_count(&self) -> i32{
        let mut count = 0;
        if self.player1.is_some(){
            count+=1;
        }
        if self.player2.is_some(){
            count+=1;
        }

        count
    }
}

impl Player {
    pub fn new(id: crate::id::Id, name: String) -> Self {
        Self { id, name }
    }
}
