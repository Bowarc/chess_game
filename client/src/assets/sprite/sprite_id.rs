#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, serde::Deserialize)]
pub enum SpriteId {
    #[default]
    MissingNo,
    AbilityPower,
    AttackDamage,
    ChessPiece(shared::chess::Color, shared::chess::Piece),
}
