#[derive(Copy, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BitBoard(u64);

// format!(":b", self.0) displays the raw bytes of the stored value
impl BitBoard {
    pub fn set(&mut self, position: impl Into<super::Position>) {
        let index = position.into().to_index();

        let b = BitBoard::from(1u64 << index);
        self.add(b);
    }
    pub fn unset(&mut self, position: impl Into<super::Position>) {
        let index = position.into().to_index();

        let b = BitBoard::from(1u64 << index);

        self.sub(b);
    }
    pub fn read(&self, position: impl Into<super::Position>) -> bool {
        let index = position.into().to_index();

        let mask = BitBoard(1 << index);

        // println!("Mask: {mask}");

        (self & mask).0 != 0

        // self.0 & 1 << index(row, col) != 0
    }

    pub fn add(&mut self, other: BitBoard) {
        *self |= other
    }
    pub fn sub(&mut self, other: BitBoard) {
        *self &= !other
    }

    /// Used to see the other player's perspective
    pub fn flip(&mut self) {
        // TODO https://github.com/Bowarc/chess_game/issues/29

        self.0 = self.0.reverse_bits()
    }
}

impl std::fmt::Debug for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s: String = format!("BitBoard({})\n", self.0);
        for col in (0..8).rev() {
            s.push_str(&format!("{} ", col + 1));
            for row in 0..8 {
                let pos = super::Position::from_index(row, col).unwrap();
                let index = pos.to_index();
                if self.0 & (1u64 << index) == (1u64 << index) {
                    s.push_str("X ");
                } else {
                    s.push_str("• ");
                }
                if row == 7 {
                    s.push('\n');
                }
            }
        }
        s.push_str("  A B C D E F G H");

        write!(f, "{}", s)
    }
}

impl From<u64> for BitBoard {
    fn from(v: u64) -> Self {
        BitBoard(v)
    }
}

// impl std::ops::Deref for BitBoard {
//     type Target = u64;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl std::ops::DerefMut for BitBoard {
//     fn deref_mut(&mut self) -> &mut u64 {
//         &mut self.0
//     }
// }

// AND
impl std::ops::BitAnd for BitBoard {
    type Output = BitBoard;
    fn bitand(self, rhs: BitBoard) -> BitBoard {
        BitBoard(self.0 & rhs.0)
    }
}

impl std::ops::BitAnd for &BitBoard {
    type Output = BitBoard;
    fn bitand(self, rhs: &BitBoard) -> BitBoard {
        BitBoard(self.0 & rhs.0)
    }
}

impl std::ops::BitAnd<&BitBoard> for BitBoard {
    type Output = BitBoard;
    fn bitand(self, rhs: &BitBoard) -> BitBoard {
        BitBoard(self.0 & rhs.0)
    }
}

impl std::ops::BitAnd<BitBoard> for &BitBoard {
    type Output = BitBoard;
    fn bitand(self, rhs: BitBoard) -> BitBoard {
        BitBoard(self.0 & rhs.0)
    }
}

// AND ASSIGN
impl std::ops::BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs
    }
}
impl std::ops::BitAndAssign<&BitBoard> for BitBoard {
    fn bitand_assign(&mut self, rhs: &Self) {
        *self = *self & rhs
    }
}

// OR
impl std::ops::BitOr for BitBoard {
    type Output = BitBoard;
    fn bitor(self, rhs: BitBoard) -> BitBoard {
        BitBoard(self.0 | rhs.0)
    }
}

impl std::ops::BitOr for &BitBoard {
    type Output = BitBoard;
    fn bitor(self, rhs: &BitBoard) -> BitBoard {
        BitBoard(self.0 | rhs.0)
    }
}

impl std::ops::BitOr<&BitBoard> for BitBoard {
    type Output = BitBoard;
    fn bitor(self, rhs: &BitBoard) -> BitBoard {
        BitBoard(self.0 | rhs.0)
    }
}

impl std::ops::BitOr<BitBoard> for &BitBoard {
    type Output = BitBoard;
    fn bitor(self, rhs: BitBoard) -> BitBoard {
        BitBoard(self.0 | rhs.0)
    }
}

// OR ASSIGN
impl std::ops::BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs
    }
}
impl std::ops::BitOrAssign<&BitBoard> for BitBoard {
    fn bitor_assign(&mut self, rhs: &Self) {
        *self = *self | rhs
    }
}

impl std::ops::Not for BitBoard {
    type Output = BitBoard;

    fn not(self) -> BitBoard {
        BitBoard(!self.0)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;
    #[test]
    fn set() {
        let mut b = BitBoard(0);

        b.set((0, 0));

        assert_eq!(b, BitBoard(1));

        let p = super::super::Position::from((3, 6));

        println!("Setting square {p}");

        b.set(p);

        assert_eq!(b, BitBoard(2251799813685249)) // 1073741825 With the other index method
    }

    #[test]
    fn unset() {
        let mut b = BitBoard(987654321987654321);

        b.unset((0, 0));

        assert_eq!(b, BitBoard(987654321987654320));

        b.unset((2, 4));

        assert_eq!(b, BitBoard(987654304807785136)) // 987654321986605744 With the other index method
    }

    #[test]
    fn display() {
        use super::super::{File, Position, Rank};
        let mut b = BitBoard(0);

        b.set(Position::from_file_rank(File::F, Rank::Three));

        b.set((File::A, Rank::One));
        b.set((File::A, Rank::Four));

        b.set((File::H, Rank::Eight));

        println!("{b}");
    }

    #[test]
    fn flip(){
        let mut b = BitBoard(0);

        for i in 0..4{
            let file = super::super::File::A;
            let rank = super::super::Rank::from_index(i).unwrap();
            b.set(super::super::Position::from_file_rank(file, rank));
        }

        // println!("{b}");
        assert_eq!(b, BitBoard(16843009));
        
        b.flip();

        // println!("{b}");
        assert_eq!(b, BitBoard(9259542121117908992));
    }
}
