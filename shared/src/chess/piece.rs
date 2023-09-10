#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize, Hash, PartialEq, Eq)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}
