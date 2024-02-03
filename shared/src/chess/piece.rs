pub const ALL_PIECES: [Piece; 6] = [
    Piece::Pawn,
    Piece::Knight,
    Piece::Bishop,
    Piece::Rook,
    Piece::Queen,
    Piece::King,
];

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize, Hash, PartialEq, Eq)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl Piece {
    pub fn pseudo_legal_relative_moves(&self) -> &Vec<super::movement::RelativeChessMove> {
        let movelist = super::movement::RELATIVE_MOVES.get(self);

        movelist.unwrap()
    }

    pub fn from_fen_char(c: char) -> Option<Self> {
        match c.to_ascii_lowercase() {
            'k' => Some(Piece::King),
            'q' => Some(Piece::Queen),
            'r' => Some(Piece::Rook),
            'b' => Some(Piece::Bishop),
            'n' => Some(Piece::Knight),
            'p' => Some(Piece::Pawn),
            _ => None,
        }
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
