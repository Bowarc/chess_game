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
    pub piece: super::Piece,
    pub color: super::Color,
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

impl ChessMove {
    pub fn new(
        origin: super::Position,
        target: super::Position,
        piece: super::Piece,
        color: super::Color,
    ) -> Self {
        Self {
            origin,
            target,
            color,
            piece,
        }
    }
    pub fn is_valid(&self, board: &super::Board) -> bool {
        use super::Piece;

        if board.next_to_play() != self.color {
            // Wait your turn
            trace!("Wait your turn");
            return false;
        }

        if board.read(self.origin) != Some((self.color, self.piece)) {
            trace!("The given origin doesn't contains the given piece");
            return false;
        }

        if board
            .read(self.target)
            .map(|(color, _)| color == self.color)
            == Some(true)
        {
            // Cannot eat teammate
            trace!("Cannot eat teammate");
            return false;
        }

        // Check if the piece can move like that
        let move_delta = self.target - self.origin;

        match self.piece {
            Piece::King => {
                if !matches!(move_delta, (-1..=1, -1..=1)){
                    // Knings cannot move like that
                    trace!("Kings cannot move this way: {move_delta:?}");
                    return false;
                }
            },
            Piece::Queen => {

            },
            Piece::Rook => {

            },
            Piece::Bishop => {

            },
            Piece::Knight => {

            },
            Piece::Pawn => {

            },
        }

        false
    }
}

impl From<(i8, i8)> for RelativeMove {
    fn from(value: (i8, i8)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}
