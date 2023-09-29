#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub struct Game{
    id: crate::id::Id,
    players: Option<Player>
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub struct Player{
    id: crate::id::Id,
    name: String,
}