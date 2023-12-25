/*

    Note to understand how this should work

    let mut b = BitBoard(0);

    b.set((3, 6));

    Should make the board b:
    BitBoard(2251799813685249)
    8 • • • • • • • •
    7 • • • X • • • •
    6 • • • • • • • •
    5 • • • • • • • •
    4 • • • • • • • •
    3 • • • • • • • •
    2 • • • • • • • •
    1 • • • • • • • •
      A B C D E F G H
    Because its setting the square D7
*/
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, serde::Deserialize, serde::Serialize,)]
pub struct Position {
    file: File, // x
    rank: Rank, // y
}

// X
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, serde::Deserialize, serde::Serialize,)]
pub enum File {
    #[default]
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

// Y
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, serde::Deserialize, serde::Serialize,)]
pub enum Rank {
    #[default]
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl Position {
    pub fn from_file_rank(file: File, rank: Rank) -> Self {
        Self { file, rank }
    }
    pub fn from_index(x: impl Into<u8>, y: impl Into<u8>) -> Option<Self> {
        Some(Self {
            file: File::from_index(x)?,
            rank: Rank::from_index(y)?,
        })
    }
    pub fn to_index(&self) -> u8 {
        self.rank.to_index() * 8 + self.file.to_index()
        // self.rank.to_index() << 3 ^ self.file.to_index() // crates.rs/chess method
    }

    pub fn file(&self) -> File {
        self.file
    }
    pub fn set_file(&mut self, new_file: File) {
        self.file = new_file
    }

    pub fn rank(&self) -> Rank {
        self.rank
    }
    pub fn new_rank(&mut self, new_rank: Rank) {
        self.rank = new_rank
    }

    pub fn move_up(&mut self, amnt: impl Into<u8>) {
        let amnt = amnt.into();
        self.rank += amnt
    }
    pub fn move_down(&mut self, amnt: impl Into<u8>) {
        let amnt = amnt.into();
        self.rank -= amnt
    }
    pub fn move_left(&mut self, amnt: impl Into<u8>) {
        let amnt = amnt.into();
        self.file -= amnt;
    }
    pub fn move_right(&mut self, amnt: impl Into<u8>) {
        let amnt = amnt.into();
        self.file += amnt
    }
}

impl From<super::Square> for super::Position {
    fn from(sq: super::Square) -> Self {
        Position {
            file: sq.file(),
            rank: sq.rank(),
        }
    }
}

impl File {
    pub fn from_index(i: impl Into<u8>) -> Option<Self> {
        match i.into() {
            0 => Some(File::A),
            1 => Some(File::B),
            2 => Some(File::C),
            3 => Some(File::D),
            4 => Some(File::E),
            5 => Some(File::F),
            6 => Some(File::G),
            7 => Some(File::H),
            _ => None,
        }
    }
    pub fn to_index(&self) -> u8 {
        match self {
            File::A => 0,
            File::B => 1,
            File::C => 2,
            File::D => 3,
            File::E => 4,
            File::F => 5,
            File::G => 6,
            File::H => 7,
        }
    }
}

impl Rank {
    pub fn from_index(i: impl Into<u8>) -> Option<Self> {
        match i.into() {
            0 => Some(Rank::One),
            1 => Some(Rank::Two),
            2 => Some(Rank::Three),
            3 => Some(Rank::Four),
            4 => Some(Rank::Five),
            5 => Some(Rank::Six),
            6 => Some(Rank::Seven),
            7 => Some(Rank::Eight),
            _ => None,
        }
    }
    pub fn to_index(&self) -> u8 {
        match self {
            Rank::One => 0,
            Rank::Two => 1,
            Rank::Three => 2,
            Rank::Four => 3,
            Rank::Five => 4,
            Rank::Six => 5,
            Rank::Seven => 6,
            Rank::Eight => 7,
        }
    }
}

impl std::ops::Add<u8> for File {
    type Output = File;

    fn add(self, rhs: u8) -> Self::Output {
        File::from_index((self.to_index() + rhs) % 8).unwrap()
    }
}

impl std::ops::AddAssign<u8> for File {
    fn add_assign(&mut self, rhs: u8) {
        *self = *self + rhs
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl std::ops::Sub<u8> for File {
    type Output = File;

    fn sub(self, rhs: u8) -> Self::Output {
        File::from_index(self.to_index().wrapping_sub(rhs) % 8).unwrap()
    }
}

impl std::ops::SubAssign<u8> for File {
    fn sub_assign(&mut self, rhs: u8) {
        *self = *self - rhs
    }
}

impl std::ops::Add<u8> for Rank {
    type Output = Rank;

    fn add(self, rhs: u8) -> Self::Output {
        Rank::from_index((self.to_index() + rhs) % 8).unwrap()
    }
}
impl std::ops::AddAssign<u8> for Rank {
    fn add_assign(&mut self, rhs: u8) {
        *self = *self + rhs
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl std::ops::Sub<u8> for Rank {
    type Output = Rank;

    fn sub(self, rhs: u8) -> Self::Output {
        Rank::from_index(self.to_index().wrapping_sub(rhs) % 8).unwrap()
    }
}

impl std::ops::SubAssign<u8> for Rank {
    fn sub_assign(&mut self, rhs: u8) {
        *self = *self - rhs
    }
}

impl From<(u8, u8)> for Position {
    fn from(value: (u8, u8)) -> Self {
        Self::from_index(value.0, value.1).unwrap()
    }
}

impl From<(super::File, super::Rank)> for Position {
    fn from(value: (super::File, super::Rank)) -> Self {
        Position::from_file_rank(value.0, value.1)
    }
}

impl ToString for File {
    fn to_string(&self) -> String {
        match self {
            File::A => String::from("A"),
            File::B => String::from("B"),
            File::C => String::from("C"),
            File::D => String::from("D"),
            File::E => String::from("E"),
            File::F => String::from("F"),
            File::G => String::from("G"),
            File::H => String::from("H"),
        }
    }
}

impl ToString for Rank {
    fn to_string(&self) -> String {
        match self {
            Rank::One => String::from("1"),
            Rank::Two => String::from("2"),
            Rank::Three => String::from("3"),
            Rank::Four => String::from("4"),
            Rank::Five => String::from("5"),
            Rank::Six => String::from("6"),
            Rank::Seven => String::from("7"),
            Rank::Eight => String::from("8"),
        }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file.to_string(), self.rank.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rank_add() {
        let r = Rank::One;

        assert_eq!(r + 5, Rank::Six);
        assert_eq!(r + 0, Rank::One);
        assert_eq!(r + 8, Rank::One);
    }

    #[test]
    fn rank_sub() {
        let r = Rank::Eight;

        assert_eq!(r - 2, Rank::Six);
        assert_eq!(r - 0, Rank::Eight);
        assert_eq!(r - 8, Rank::Eight);
    }

    #[test]
    fn file_add() {
        let f = File::A;

        assert_eq!(f + 5, File::F);

        assert_eq!(f + 0, File::A);

        assert_eq!(f + 8, File::A);
    }

    #[test]
    fn file_sub() {
        let f = File::H;

        assert_eq!(f - 2, File::F);

        assert_eq!(f - 0, File::H);
        assert_eq!(f - 8, File::H);
    }

    #[test]
    fn index() {
        assert_eq!(Position::from_index(2, 4).unwrap().to_index(), 34);

        assert_eq!(Position::from_index(3, 6).unwrap().to_index(), 51);

        assert_eq!(Position::from_index(0, 0).unwrap().to_index(), 0);

        assert_eq!(Position::from_index(7, 7).unwrap().to_index(), 63);

        assert_eq!(Position::from_index(8, 8), None)
    }

    #[test]
    fn r#move() {
        let mut pos = Position::from_index(0, 7).unwrap();

        println!("{pos}");
        pos.move_down(1);
        println!("{pos}");
    }
}
