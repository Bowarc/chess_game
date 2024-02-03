lazy_static::lazy_static! {
    pub static ref RELATIVE_MOVES: std::collections::HashMap<super::Piece, Vec<RelativeChessMove>> = {
        let path = crate::file::Path::new(crate::file::FileSystem::Internal, "config/pieces_relative_moves.ron".to_string());
        let bytes = crate::file::bytes(path);
        let x = ron::de::from_bytes::<std::collections::HashMap<super::Piece, Vec<RelativeChessMove>>>(&bytes).unwrap();
        info!("Loaded relative chess moves: {x:?}");
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
pub struct RelativeChessMove {
    pub x: i8,
    pub y: i8,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Hash, PartialEq, Eq)]
pub struct RelativeMoveList {
    normal: Vec<RelativeChessMove>,
    eat: Vec<RelativeChessMove>,
    specific: Vec<RelativeChessMove>,
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
    pub fn is_pseudo_legal(&self, board: &super::Board) -> bool {
        if board.next_to_play() != self.color {
            // Wait your turn
            debug!("Wait your turn");
            return false;
        }

        if board.read(self.origin) != Some((self.color, self.piece)) {
            debug!("The given origin doesn't contains the given piece");
            return false;
        }

        if board
            .read(self.target)
            .map(|(color, _)| color == self.color)
            == Some(true)
        {
            // Cannot eat teammate
            debug!("Cannot eat teammate");
            return false;
        }

        // Check if the piece can move like that
        let relative_move = self.relative();

        if !self
            .piece
            .pseudo_legal_relative_moves()
            .contains(&relative_move)
        {
            // This piece cannot move like that
            debug!(
                "{piece} cannot move this way: {relative_move:?}",
                piece = self.piece
            );
            return false;
        }

        true
    }

    pub fn relative(&self) -> RelativeChessMove {
        // I belive that we shoud reverse it if the player is black, as it's a perspective

        let mut relative_mv = self.target - self.origin;

        if self.color == super::Color::Black {
            relative_mv.y *= -1;
            relative_mv.x *= -1;
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
