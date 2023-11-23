#[rustfmt::skip]
pub enum Square {
    A1,A2,A3,A4,A5,A6,A7,A8,
    B1,B2,B3,B4,B5,B6,B7,B8,
    C1,C2,C3,C4,C5,C6,C7,C8,
    D1,D2,D3,D4,D5,D6,D7,D8,
    E1,E2,E3,E4,E5,E6,E7,E8,
    F1,F2,F3,F4,F5,F6,F7,F8,
    G1,G2,G3,G4,G5,G6,G7,G8,
    H1,H2,H3,H4,H5,H6,H7,H8,
}

impl Square {
    #[rustfmt::skip]
    pub fn file(&self) -> super::File{
        match self {
            Square::A1|Square::A2|Square::A3|Square::A4|Square::A5|Square::A6|Square::A7|Square::A8 => {
                super::File::A
            },
            Square::B1|Square::B2|Square::B3|Square::B4|Square::B5|Square::B6|Square::B7|Square::B8 => {
                super::File::B
            },
            Square::C1|Square::C2|Square::C3|Square::C4|Square::C5|Square::C6|Square::C7|Square::C8 => {
                super::File::C
            },
            Square::D1|Square::D2|Square::D3|Square::D4|Square::D5|Square::D6|Square::D7|Square::D8 => {
                super::File::D
            },
            Square::E1|Square::E2|Square::E3|Square::E4|Square::E5|Square::E6|Square::E7|Square::E8 => {
                super::File::E
            },
            Square::F1|Square::F2|Square::F3|Square::F4|Square::F5|Square::F6|Square::F7|Square::F8 => {
                super::File::F
            },
            Square::G1|Square::G2|Square::G3|Square::G4|Square::G5|Square::G6|Square::G7|Square::G8 => {
                super::File::G
            },
            Square::H1|Square::H2|Square::H3|Square::H4|Square::H5|Square::H6|Square::H7|Square::H8 => {
                super::File::H
            },
        }
    }
    pub fn rank(&self) -> super::Rank {
        match self {
            Square::A1
            | Square::B1
            | Square::C1
            | Square::D1
            | Square::E1
            | Square::F1
            | Square::G1
            | Square::H1 => super::Rank::One,
            Square::A2
            | Square::B2
            | Square::C2
            | Square::D2
            | Square::E2
            | Square::F2
            | Square::G2
            | Square::H2 => super::Rank::Two,
            Square::A3
            | Square::B3
            | Square::C3
            | Square::D3
            | Square::E3
            | Square::F3
            | Square::G3
            | Square::H3 => super::Rank::Three,
            Square::A4
            | Square::B4
            | Square::C4
            | Square::D4
            | Square::E4
            | Square::F4
            | Square::G4
            | Square::H4 => super::Rank::Four,
            Square::A5
            | Square::B5
            | Square::C5
            | Square::D5
            | Square::E5
            | Square::F5
            | Square::G5
            | Square::H5 => super::Rank::Five,
            Square::A6
            | Square::B6
            | Square::C6
            | Square::D6
            | Square::E6
            | Square::F6
            | Square::G6
            | Square::H6 => super::Rank::Six,
            Square::A7
            | Square::B7
            | Square::C7
            | Square::D7
            | Square::E7
            | Square::F7
            | Square::G7
            | Square::H7 => super::Rank::Seven,
            Square::A8
            | Square::B8
            | Square::C8
            | Square::D8
            | Square::E8
            | Square::F8
            | Square::G8
            | Square::H8 => super::Rank::Eight,
        }
    }
}

impl From<super::Position> for Square {
    fn from(pos: super::Position) -> Square {
        match pos.file() {
            super::File::A => match pos.rank() {
                super::Rank::One => Square::A1,
                super::Rank::Two => Square::A2,
                super::Rank::Three => Square::A3,
                super::Rank::Four => Square::A4,
                super::Rank::Five => Square::A5,
                super::Rank::Six => Square::A6,
                super::Rank::Seven => Square::A7,
                super::Rank::Eight => Square::A8,
            },
            super::File::B => match pos.rank() {
                super::Rank::One => Square::B1,
                super::Rank::Two => Square::B2,
                super::Rank::Three => Square::B3,
                super::Rank::Four => Square::B4,
                super::Rank::Five => Square::B5,
                super::Rank::Six => Square::B6,
                super::Rank::Seven => Square::B7,
                super::Rank::Eight => Square::B8,
            },
            super::File::C => match pos.rank() {
                super::Rank::One => Square::C1,
                super::Rank::Two => Square::C2,
                super::Rank::Three => Square::C3,
                super::Rank::Four => Square::C4,
                super::Rank::Five => Square::C5,
                super::Rank::Six => Square::C6,
                super::Rank::Seven => Square::C7,
                super::Rank::Eight => Square::C8,
            },
            super::File::D => match pos.rank() {
                super::Rank::One => Square::D1,
                super::Rank::Two => Square::D2,
                super::Rank::Three => Square::D3,
                super::Rank::Four => Square::D4,
                super::Rank::Five => Square::D5,
                super::Rank::Six => Square::D6,
                super::Rank::Seven => Square::D7,
                super::Rank::Eight => Square::D8,
            },
            super::File::E => match pos.rank() {
                super::Rank::One => Square::E1,
                super::Rank::Two => Square::E2,
                super::Rank::Three => Square::E3,
                super::Rank::Four => Square::E4,
                super::Rank::Five => Square::E5,
                super::Rank::Six => Square::E6,
                super::Rank::Seven => Square::E7,
                super::Rank::Eight => Square::E8,
            },
            super::File::F => match pos.rank() {
                super::Rank::One => Square::F1,
                super::Rank::Two => Square::F2,
                super::Rank::Three => Square::F3,
                super::Rank::Four => Square::F4,
                super::Rank::Five => Square::F5,
                super::Rank::Six => Square::F6,
                super::Rank::Seven => Square::F7,
                super::Rank::Eight => Square::F8,
            },
            super::File::G => match pos.rank() {
                super::Rank::One => Square::G1,
                super::Rank::Two => Square::G2,
                super::Rank::Three => Square::G3,
                super::Rank::Four => Square::G4,
                super::Rank::Five => Square::G5,
                super::Rank::Six => Square::G6,
                super::Rank::Seven => Square::G7,
                super::Rank::Eight => Square::G8,
            },
            super::File::H => match pos.rank() {
                super::Rank::One => Square::H1,
                super::Rank::Two => Square::H2,
                super::Rank::Three => Square::H3,
                super::Rank::Four => Square::H4,
                super::Rank::Five => Square::H5,
                super::Rank::Six => Square::H6,
                super::Rank::Seven => Square::H7,
                super::Rank::Eight => Square::H8,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_pos() {
        let pos: super::super::Position = Square::E1.into();
    }
}
