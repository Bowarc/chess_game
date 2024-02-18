lazy_static::lazy_static! {
    pub static ref RELATIVE_MOVES: std::collections::HashMap<super::Piece, Vec<RelativeChessMove>> = {
        let path = crate::file::Path::new(crate::file::FileSystem::Internal, "config/pieces_relative_moves.ron".to_string());
        let bytes = crate::file::bytes(path);
        ron::de::from_bytes::<std::collections::HashMap<super::Piece, Vec<RelativeChessMove>>>(&bytes).unwrap()
    };
}

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize, Hash, PartialEq, Eq)]
pub struct ChessMove {
    pub origin: super::Position,
    pub target: super::Position,
    pub piece: super::Piece,
    pub color: super::Color,
    pub promotion: Option<super::Piece>,
    // castle: bool,
    // eat: Option<super::Position>, // could be bool but the eaten piece is not at the target pos if en-passant, right ?
}

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize, Hash, PartialEq, Eq)]
#[serde(from = "(i8, i8)")]
pub struct RelativeChessMove {
    pub x: i8,
    pub y: i8,
}

impl ChessMove {
    pub fn new(
        origin: super::Position,
        target: super::Position,
        piece: super::Piece,
        color: super::Color,
        promotion: Option<super::Piece>,
    ) -> Self {
        Self {
            origin,
            target,
            color,
            piece,
            promotion,
        }
    }

    pub fn is_legal(&self, board: &super::Board) -> bool {
        super::movegen::all_legals(self.piece, self.origin, board)
            .unwrap_or_default()
            .contains(self)
    }

    pub fn relative(&self) -> RelativeChessMove {
        // I belive that we shoud reverse it if the player is black, as it's a perspective

        let mut relative_mv = self.target - self.origin;

        if self.color == super::Color::Black {
            relative_mv.x *= -1;
            relative_mv.y *= -1;
        }

        relative_mv
    }
}

impl From<(i8, i8)> for RelativeChessMove {
    fn from(value: (i8, i8)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<RelativeChessMove> for (i8, i8) {
    fn from(mv: RelativeChessMove) -> Self {
        (mv.x, mv.y)
    }
}
