pub struct Board{
    active_player: super::Color,
    pieces: crate::maths::Vec2D<Option<super::Piece>>

}

impl Board{
    // pub fn 
}


impl Default for Board{
    fn default() -> Self{
        Self{
            active_player: super::Color::White, // White always starts
            pieces: crate::maths::Vec2D::new_empty() // Make a new from fen string
        }
    }
}