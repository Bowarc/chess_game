#[derive(Copy, Clone, PartialEq, Default)]
pub struct BitBoard(u64);

// format!(":b", self.0) displays the raw bytes of the stored value
impl BitBoard {
    pub fn set(&mut self, row: impl Into<i32>, col: impl Into<i32>) {
        let index = index(row, col);

        println!("Setting {index}");

        let b = BitBoard::from(1u64 << index);
        self.add(b);
    }
    pub fn unset(&mut self, row: impl Into<i32>, col: impl Into<i32>) {
        let index = index(row, col);

        println!("Unsetting {index}");
        let b = BitBoard::from(1u64 << index);

        self.sub(b);
    }
    pub fn read(&self, row: impl Into<i32>, col: impl Into<i32>) -> bool {
        let index = index(row, col);

        let mask = BitBoard(1 << index);

        println!("Mask: {mask}");

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
        self.0 = self.0.swap_bytes()
    }
}

pub fn index(row: impl Into<i32>, col: impl Into<i32>) -> i32 {
    let row = row.into();
    let col = col.into();
    row * 8 + col
}

impl std::fmt::Debug for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for BitBoard {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s: String = format!("BitBoard({})\n", self.0);

        for row in (0..8).rev() {
            for col in 0..8 {
                let index = index(row, col);
                if self.0 & (1u64 << index) == (1u64 << index) {
                    s.push_str("X ");
                } else {
                    s.push_str("• ");
                }
                if col % 8 == 7 {
                    s.push('\n');
                }
            }
        }

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
    use super::*;

    #[test]
    fn index() {
        assert_eq!(super::index(2, 4), 20);

        assert_eq!(super::index(3, 6), 30);

        assert_eq!(super::index(0, 0), 0);
    }

    #[test]
    fn set() {
        let mut b = BitBoard(0);

        b.set(0, 0);

        assert_eq!(b, BitBoard(1));

        b.set(3, 6);

        assert_eq!(b, BitBoard(1073741825))
    }

    #[test]
    fn unset() {
        let mut b = BitBoard(987654321987654321);

        b.unset(0, 0);

        assert_eq!(b, BitBoard(987654321987654320));

        b.unset(2, 4);

        assert_eq!(b, BitBoard(987654321986605744))
    }
}

// #[test]
// fn bboard() {
//     let mut b = BitBoard(987654321987654321);

//     println!("{b}");

//     b.set(0, 0);
//     println!("{b}");

//     b.unset(0, 0);
//     println!("{b}");

//     b.set(0, 0);
//     println!("{b}");

//     println!("{}", b.read(7, 1))
// }
