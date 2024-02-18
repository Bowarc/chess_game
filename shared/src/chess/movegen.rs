pub fn all_legals(
    piece: super::Piece,
    piece_pos: super::Position,
    board: &super::Board,
) -> Option<Vec<super::ChessMove>> {
    let mut out = Vec::new();

    let piece_color = {
        let (read_color, read_piece) = board.read(piece_pos)?;

        if read_piece != piece {
            return None;
        }
        read_color
    };

    out.append(&mut basic_moves(piece, piece_pos, piece_color));


    // Specific moves
    match piece {
        super::Piece::King => {}
        super::Piece::Rook => {}
        super::Piece::Pawn => {
            out.append(&mut pawn_first_move(piece, piece_pos, piece_color));

            out.append(&mut pawn_eat(piece, piece_pos, piece_color, board));
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
            if !los_filter(**mv, board) {
                return false;
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
        .collect::<Vec<super::ChessMove>>();

    debug!("All legal moves for {piece_color} {piece}s: {out:#?}");
    Some(out)
}

fn basic_moves(
    piece: super::Piece,
    piece_pos: super::Position,
    piece_color: super::Color,
) -> Vec<super::ChessMove> {
    piece
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
            Some(super::ChessMove::new(
                piece_pos,
                target,
                piece,
                piece_color,
                None,
            ))
        })
        .collect::<Vec<super::ChessMove>>()
}

fn pawn_first_move(
    piece: super::Piece,
    piece_pos: super::Position,
    piece_color: super::Color,
) -> Vec<super::ChessMove> {
    let delta: i8 = if piece_pos.rank() == super::Rank::Two && piece_color == super::Color::White {
        2
    } else if piece_pos.rank() == super::Rank::Seven && piece_color == super::Color::Black {
        -2
    } else {
        warn!("Pawn first move check else");
        return Vec::new();
    };

    let Some(target) = super::Position::from_index(
        piece_pos.file().to_index(),
        (piece_pos.rank().to_index() as i8 + delta) as u8,
    ) else {
        return Vec::new();
    };

    vec![super::ChessMove::new(
        piece_pos,
        target,
        piece,
        piece_color,
        None,
    )]
}

fn pawn_eat(
    piece: super::Piece,
    piece_pos: super::Position,
    piece_color: super::Color,
    board: &super::Board,
) -> Vec<super::ChessMove> {
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
    .flat_map(|target| {
        // Can't eat air
        let Some((read_color, _read_piece)) = board.read(*target) else {
            return None;
        };

        // Can't eat teammate
        if read_color == piece_color {
            return None;
        }

        Some(super::ChessMove::new(
            piece_pos,
            *target,
            piece,
            piece_color,
            None,
        ))
    })
    .collect::<Vec<super::ChessMove>>()
}

// Make sure that a piece won't go through another one
fn los_filter(mv: super::ChessMove, board: &super::Board) -> bool {
    // Bypass for knight as the line of sight check is useless and the target check done above
    if mv.piece == super::Piece::Knight {
        return true;
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

        if let Some((read_color, _)) = board.read(pos.into()) {
            // You can eat, but you can't go further
            if read_color != mv.color && pos == target {
                return true;
            }
            // You can't eat your allies
            debug!("You can't eat your allies");
            return false;
        }

        if pos == target {
            return true;
        }
    }
}
