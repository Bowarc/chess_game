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

    pub fn add(&mut self, other: BitBoard) {
        self.0 |= other.0
    }
    pub fn sub(&mut self, other: BitBoard) {
        self.0 &= !other.0
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

#[test]
fn bboard() {
    let mut b = BitBoard(987654321987654321);

    println!("{b}");

    b.set(0, 0);
    println!("{b}");

    b.unset(0, 0);
    println!("{b}")
}
