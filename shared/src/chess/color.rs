#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Color {
    Black,
    #[default]
    White,
}

impl Color {
    pub fn from_fen_char(c: char) -> Self {
        // Do we throw an error if the char can't be translated into a piece ?
        // assert!(super::Piece::from_fen_char(c).is_some())

        if c.is_ascii_uppercase() {
            Color::White
        } else {
            Color::Black
        }
    }
}

impl std::ops::Not for Color {
    type Output = Color;

    fn not(self) -> Color {
        if self == Color::White {
            Color::Black
        } else {
            Color::White
        }
    }
}
