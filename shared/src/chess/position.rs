#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Position {
    file: File,
    rank: Rank,
}

// X
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
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
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
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
    pub fn from_xy(x: impl Into<u8>, y: impl Into<u8>) -> Option<Self> {
        Some(Self {
            file: File::from_index(x)?,
            rank: Rank::from_index(y)?,
        })
    }

    pub fn x(&self) -> u8 {
        self.file.to_index()
    }

    pub fn file(&self) -> File {
        self.file
    }

    pub fn y(&self) -> u8 {
        self.rank.to_index()
    }

    pub fn rank(&self) -> Rank {
        self.rank
    }

    pub fn to_index(&self) -> u8 {
        self.x() * 8 + self.y()
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
        Self::from_xy(value.0, value.1).unwrap()
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
        assert_eq!(Position::from_xy(2, 4).unwrap().to_index(), 20);

        assert_eq!(Position::from_xy(3, 6).unwrap().to_index(), 30);

        assert_eq!(Position::from_xy(0, 0).unwrap().to_index(), 0);

        assert_eq!(Position::from_xy(7, 7).unwrap().to_index(), 63);

        assert_eq!(Position::from_xy(8, 8), None)
    }
}
