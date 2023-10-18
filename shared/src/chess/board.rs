pub struct Board {
    active_player: super::Color,
    white_bb: super::BitBoard,
    black_bb: super::BitBoard,

    piece_bb: hashbrown::HashMap<super::Piece, super::BitBoard>,
}

impl Board {
    pub fn from_fen(fen: &str) -> Option<Self> {
        let mut board = Self {
            active_player: super::Color::default(), // White always starts (Unless the FEN string says otherwise)
            white_bb: super::BitBoard::default(),
            black_bb: super::BitBoard::default(),
            piece_bb: hashbrown::HashMap::default(),
        };

        // Initialize the pieces bitboards
        for piece in super::piece::ALL_PIECES {
            board.piece_bb.insert(piece, super::BitBoard::default());
        }

        let tokens = fen
            .split(' ')
            .map(|split| split.to_string())
            .collect::<Vec<String>>();

        if tokens.len() < 4 {
            error!("Invalid FEN string ({fen})");
            return None;
        }

        let pieces = tokens.get(0).unwrap();
        let active_player = tokens.get(1).unwrap();
        let _castles = tokens.get(2).unwrap();
        let _idk = tokens.get(3).unwrap();

        // Set all the pieces to the right places
        let mut pos = super::Position::from_index(0, 7).unwrap();
        for p in pieces.chars() {
            // Check line end
            if p == '/' {
                pos.set_file(super::File::A);
                pos.move_down(1);
                continue;
            }

            // check nbr
            if let Some(nbr) = p.to_digit(10) {
                pos.move_right(nbr as u8);
                continue;
            }

            // Else, match the piece or return an error if it's no understood
            let Some(piece) = super::Piece::from_fen_char(p) else {
                error!("Could not convert FEN '{p}' to piece");
                return None;
            };

            board.set(
                piece,
                super::Color::from_fen_char(p),
                super::Position::from_file_rank(pos.file(), pos.rank()),
            );
            pos.move_right(1);
        }

        // 'Deserialize' the active player
        if active_player.len() != 1 {
            error!("Could not understand the active player of fen ({fen})");
            return None;
        }
        match active_player.as_str() {
            "w" => board.active_player = super::Color::White,
            "b" => board.active_player = super::Color::Black,
            _ => {
                error!("Could not get the active player from fen string ({fen})");
                return None;
            }
        };

        Some(board)
    }

    pub fn set(&mut self, piece: super::Piece, color: super::Color, pos: super::Position) {
        let color_bb = match color {
            super::Color::Black => &mut self.black_bb,
            super::Color::White => &mut self.white_bb,
        };

        color_bb.set(pos);

        let piece_bb = self.piece_bb.get_mut(&piece).unwrap();
        piece_bb.set(pos);
    }

    pub fn unset(&mut self, piece: super::Piece, color: super::Color, pos: super::Position) {
        let color_bb = match color {
            super::Color::Black => &mut self.black_bb,
            super::Color::White => &mut self.white_bb,
        };

        color_bb.unset(pos);

        let piece_bb = self.piece_bb.get_mut(&piece).unwrap();
        piece_bb.unset(pos);
    }

    pub fn flip(&mut self) {
        self.white_bb.flip();
        self.black_bb.flip();
        for (_piece, bb) in self.piece_bb.iter_mut() {
            bb.flip();
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }
}

// impl std::fmt::Display for Board {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         let mut s = String::new();

//         write!(f, "{}", s)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fen() {
        let mut b =
            Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

        assert_eq!(b.active_player, super::super::Color::White);

        fn display(b: &Board) {
            println!("Whites: {}", b.white_bb);
            println!("Blacks: {}", b.black_bb);

            for piece in super::super::piece::ALL_PIECES {
                println!("{piece:?} {}", b.piece_bb.get(&piece).unwrap())
            }
        }
        display(&b);

        println!("Flipping");
        b.flip();

        display(&b);

        println!("{}", 554050781184u64.swap_bytes());
    }
}
