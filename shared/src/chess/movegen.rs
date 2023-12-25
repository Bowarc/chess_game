pub enum Condition{
    FirstMove,
    // Castle,
    // EnPassant,
}

pub fn all_conditions(_board: &super::Board) -> Vec<Condition>{
    Vec::new()
}

pub fn is_check(_board: &super::Board, color: super::Color) -> bool{
    false
}

pub fn legal_moves(){}