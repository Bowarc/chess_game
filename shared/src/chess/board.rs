#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub struct Board {
    active_player: super::Color,
    pub white_bb: super::BitBoard,
    pub black_bb: super::BitBoard,

    piece_bb: std::collections::HashMap<super::Piece, super::BitBoard>,
}

impl Board {
    pub fn from_fen(fen: &str) -> Option<Self> {
        let mut board = Self {
            active_player: super::Color::default(), // White always starts (Unless the FEN string says otherwise)
            white_bb: super::BitBoard::default(),
            black_bb: super::BitBoard::default(),
            piece_bb: std::collections::HashMap::default(),
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

            board.set(piece, super::Color::from_fen_char(p), pos);
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

    pub fn next_to_play(&self) -> super::Color {
        self.active_player
    }

    pub fn make_move(&mut self, mv: &super::movement::ChessMove) -> Result<(), ()> {
        if mv.color != self.active_player {
            return Err(());
        }
        let color_bb = match mv.color {
            super::Color::Black => self.black_bb,
            super::Color::White => self.white_bb,
        };

        let bb = self.piece_bb.get(&mv.piece).unwrap() & color_bb;

        if !bb.read(mv.origin) {
            // There is no given piece at that position
            return Err(());
        }

        // Just overwrite the target pos for now
        self.unset(mv.piece, mv.color, mv.origin);

        if let Some((color, piece )) = self.read(mv.target){
            self.unset(piece, color, mv.target); // Forgetting this causes a panic in the next read, aaand #46
        }
        self.set(mv.piece, mv.color, mv.target);

        self.active_player = !self.active_player;
        Ok(())
    }

    fn set(&mut self, piece: super::Piece, color: super::Color, pos: super::Position) {
        let color_bb = match color {
            super::Color::Black => &mut self.black_bb,
            super::Color::White => &mut self.white_bb,
        };

        color_bb.set(pos);

        let piece_bb = self.piece_bb.get_mut(&piece).unwrap();
        piece_bb.set(pos);
    }

    fn unset(&mut self, piece: super::Piece, color: super::Color, pos: super::Position) {
        let color_bb = match color {
            super::Color::Black => &mut self.black_bb,
            super::Color::White => &mut self.white_bb,
        };

        color_bb.unset(pos);

        let piece_bb = self.piece_bb.get_mut(&piece).unwrap();
        piece_bb.unset(pos);
    }

    pub fn read(&self, pos: super::Position) -> Option<(super::Color, super::Piece)> {
        if !(self.white_bb | self.black_bb).read(pos) {
            // Not in any board
            return None;
        }

        let piece: Vec<&super::Piece> = self
            .piece_bb
            .iter()
            .flat_map(|(p, bb)| if bb.read(pos) { Some(p) } else { None })
            .collect();

        if piece.is_empty() {
            panic!("Could not query the piece for position: {pos:?} with board: {self:?}")
        }

        let mut color = None;

        // We could assume that if it's not one it's the other, but i wanna make sure that i did not fuck up something in board sync

        if self.white_bb.read(pos) {
            color = Some(super::Color::White);
        }

        if self.black_bb.read(pos) {
            if color.is_some() {
                panic!("The position {pos} for board {self:?} is black and white");
            }
            color = Some(super::Color::Black);
        }

        Some((color.unwrap(), **piece.first().unwrap()))
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

    fn display(b: &Board) {
        println!("Whites: {}", b.white_bb);
        println!("Blacks: {}", b.black_bb);

        for piece in super::super::piece::ALL_PIECES {
            println!("{piece:?} {}", b.piece_bb.get(&piece).unwrap())
        }
    }

    #[test]
    fn fen() {
        let b =
            Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

        assert_eq!(b.active_player, super::super::Color::White);

        display(&b);
    }

    #[test]
    fn flip() {
        let mut b =
            Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

        assert_eq!(b.active_player, super::super::Color::White);

        display(&b);
        println!("Flipping");
        b.flip();

        display(&b);

        println!("{}", 554050781184u64.reverse_bits());
    }
    #[test]
    fn play() {
        use super::super::{
            position::{File, Position, Rank},
            ChessMove, Color, Piece,
        };

        let mut b = Board::default();
        let s = Position::from_file_rank(File::A, Rank::Two);
        let e = Position::from_file_rank(File::B, Rank::Three);
        b.make_move(&ChessMove::new(s, e, Piece::Pawn, Color::White))
            .unwrap();

        println!("{}", b.white_bb | b.black_bb);
    }

    #[test]
    fn show(){
        use super::super::*;
        let mut bb = BitBoard::from(35747322042318592);
        let mut black = BitBoard::from(18446462598732840960);
        let mut white = BitBoard::from(281474976775935);
        println!("{}", bb & white);
        println!("{}", bb & black);
        bb.flip();

        println!("{}", white);
        println!("{}", black);
    }
}
