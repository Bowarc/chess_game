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
    ) -> Self {
        Self {
            origin,
            target,
            color,
            piece,
        }
    }

    pub fn get_all_legals(
        piece: super::Piece,
        piece_pos: super::Position,
        board: &super::Board,
    ) -> Option<Vec<Self>> {
        let mut out = Vec::new();

        let piece_color = {
            let (read_color, read_piece) = board.read(piece_pos)?;

            if read_piece != piece {
                return None;
            }
            read_color
        };

        out.append(
            &mut piece
                .pseudo_legal_relative_moves()
                .iter()
                .flat_map(|rmv| {
                    let mut rmv = *rmv;
                    if piece_color == super::Color::Black {
                        rmv.y *= -1; // Flip it as black pieces move downwards
                    }
                    let target = super::Position::from_index(
                        (piece_pos.file().to_index() as i8 + rmv.x) as u8,
                        (piece_pos.rank().to_index() as i8 + rmv.y) as u8,
                    )?;
                    Some(ChessMove::new(piece_pos, target, piece, piece_color))
                })
                .collect::<Vec<ChessMove>>(),
        );

        debug!("pseudo legal relative moves as chess moves:\n{out:#?}");

        // Specific moves
        match piece {
            super::Piece::King => {}
            super::Piece::Rook => {}
            super::Piece::Pawn => {
                // First move
                'fm: {
                    let delta: i8 = if piece_pos.rank() == super::Rank::Two
                        && piece_color == super::Color::White
                    {
                        2
                    } else if piece_pos.rank() == super::Rank::Seven
                        && piece_color == super::Color::Black
                    {
                        -2
                    } else {
                        warn!("Pawn first move check else");
                        break 'fm;
                    };

                    let Some(target) = super::Position::from_index(
                        piece_pos.file().to_index(),
                        (piece_pos.rank().to_index() as i8 + delta) as u8,
                    ) else {
                        break 'fm;
                    };

                    out.push(ChessMove::new(piece_pos, target, piece, piece_color))
                }

                // eat
                'eat: {
                    let flip: i8 = if piece_color == super::Color::White {
                        1
                    } else {
                        -1
                    };

                    [
                        super::Position::from_index(
                            (piece_pos.file().to_index() as i8 + -1) as u8,
                            (piece_pos.rank().to_index() as i8 + flip) as u8,
                        ),
                        super::Position::from_index(
                            (piece_pos.file().to_index() as i8 + 1) as u8,
                            (piece_pos.rank().to_index() as i8 + flip) as u8,
                        ),
                    ]
                    .iter()
                    .flatten() // Don't panic if the position is out of the board
                    .for_each(|target| {
                        let Some((read_color, _read_piece)) = board.read(*target) else {
                            return;
                        };

                        if read_color != piece_color {
                            out.push(ChessMove::new(piece_pos, *target, piece, piece_color));
                        }
                    });
                }
            }
            _ => {}
        }

        // Clean up the list
        let out = out
            .iter()
            .filter(|mv| {
                assert_eq!(mv.piece, piece); // if this panics, you fucked up

                // Who's turn is it ?
                if board.next_to_play() != mv.color {
                    debug!("Wait your turn");
                    return false;
                }

                // Are the infos right ?
                if board.read(mv.origin) != Some((mv.color, mv.piece)) {
                    debug!("The given origin doesn't contains the given piece");
                    return false;
                }

                // Does the target square is taken by a teammate
                if board.read(mv.target).map(|(color, _)| color == mv.color) == Some(true) {
                    debug!("Cannot eat teammate");
                    return false;
                }

                // Is the piece going over another piece ?
                'over: {
                    // Bypass for knight as the line of sight check is useless and the target check done above
                    if mv.piece == super::Piece::Knight {
                        break 'over; // I love this feature
                    }

                    let target = (mv.target.file().to_index(), mv.target.rank().to_index());
                    let origin = (mv.origin.file().to_index(), mv.origin.rank().to_index());

                    let mut pos = origin;

                    let delta = (
                        match origin.0.cmp(&target.0) {
                            std::cmp::Ordering::Less => 1,
                            std::cmp::Ordering::Equal => 0,
                            std::cmp::Ordering::Greater => -1,
                        },
                        match origin.1.cmp(&target.1) {
                            std::cmp::Ordering::Less => 1,
                            std::cmp::Ordering::Equal => 0,
                            std::cmp::Ordering::Greater => -1,
                        },
                    );

                    loop {
                        // TODO: Can this trigger an integer overflow ?
                        pos.0 = (pos.0 as i8 + delta.0) as u8;
                        pos.1 = (pos.1 as i8 + delta.1) as u8;

                        #[allow(clippy::absurd_extreme_comparisons)]
                        #[allow(unused_comparisons)]
                        if pos.0 < 0 || pos.0 > 7 || pos.1 < 0 || pos.1 > 7 {
                            return false;
                        }


                        if let Some((read_color, _)) = board.read(pos.into()){
                            // You can eat, but you can't go further
                            if read_color != mv.color && pos == target{
                                break 'over;
                            }
                            // You can't eat your allies
                            debug!("You can't eat your allies");
                            return false;
                        }

                        if pos == target {
                            break 'over;
                        }
                    }
                }

                #[allow(clippy::single_match)] // will be multiple later (maybe)
                match mv.piece {
                    crate::chess::Piece::Pawn => {
                        // Pawns cannot eat in front of them
                        if board.read(mv.target).is_some() && mv.origin.file() == mv.target.file() {
                            debug!("Pawns cannot eat in front of them");
                            return false;
                        }
                    }
                    _ => (),
                }

                true
            })
            .cloned()
            .collect::<Vec<ChessMove>>();

        debug!("Returning: {out:#?}");
        Some(out)
    }

    pub fn is_legal(&self, board: &super::Board) -> bool {
        Self::get_all_legals(self.piece, self.origin, board)
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
