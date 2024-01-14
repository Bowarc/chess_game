lazy_static::lazy_static! {
    pub static ref RELATIVE_MOVES: std::collections::HashMap<super::Piece, RelativeMoveList> = {
        let path = crate::file::Path::new(crate::file::FileSystem::Internal, "config/pieces_relative_moves.ron".to_string());
        let bytes = crate::file::bytes(path);
        let x = ron::de::from_bytes::<std::collections::HashMap<super::Piece, RelativeMoveList>>(&bytes).unwrap();
        error!("Testing: {x:?}");
        x
    };
}

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize, Hash, PartialEq, Eq)]
pub struct ChessMove {
    pub origin: super::Position,
    pub target: super::Position,
    pub color: super::Color,
    pub piece: super::Piece,
    // castle: bool,
    // eat: Option<super::Position>, // could be bool but the eaten piece is not at the target pos if en-passant, right ?
}

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize, Hash, PartialEq, Eq)]
#[serde(from = "(i8, i8)")]
pub struct RelativeMove {
    x: i8,
    y: i8,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Hash, PartialEq, Eq)]
pub struct RelativeMoveList {
    normal: Vec<RelativeMove>,
    eat: Vec<RelativeMove>,
    specific: Vec<RelativeMove>,
}

// #[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize, Hash, PartialEq, Eq)]
// pub struct AbsoluteMove {
//     piece: super::Piece,
//     origin: super::Position,
//     target: super::Position,

// }

impl From<(i8, i8)> for RelativeMove {
    fn from(value: (i8, i8)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

// pub fn validate_move(board: &super::Board, mv: &AbsoluteMove) -> bool{
//     false
// }
