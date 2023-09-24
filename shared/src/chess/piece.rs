lazy_static::lazy_static! {
    static ref RELATIVE_MOVES: std::collections::HashMap<Piece, MoveList> = {
        let path = crate::file::Path::new(crate::file::FileSystem::Internal, "config/pieces_relative_moves.ron".to_string());
        let bytes = crate::file::bytes(path);
        ron::de::from_bytes::<std::collections::HashMap<Piece, MoveList>>(&bytes).unwrap()
    };
}

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

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize, Hash, PartialEq, Eq)]
#[serde(from = "(i8, i8)")]
pub struct Move {
    x: i8,
    y: i8,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Hash, PartialEq, Eq)]
pub struct MoveList {
    normal: Vec<Move>,
    eat: Vec<Move>,
    specific: Vec<Move>,
}

impl Piece {
    pub fn relative_moves(&self) -> MoveList {
        let movelist = RELATIVE_MOVES.get(self);

        movelist.unwrap().clone()
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

impl Move {}

impl From<(i8, i8)> for Move {
    fn from(value: (i8, i8)) -> Self {
        Move {
            x: value.0,
            y: value.1,
        }
    }
}

impl MoveList {}
