#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub struct Color(u8, u8, u8, u8); // 0 - 255

impl Color {
    pub const WHITE: Self = Self(255, 255, 255, 255);
    pub const BLACK: Self = Self(0, 0, 0, 255);
    pub const TRANSPARENT: Self = Self(0, 0, 0, 0);
    pub fn from_rgba(
        r: impl Into<u8>,
        g: impl Into<u8>,
        b: impl Into<u8>,
        a: impl Into<u8>,
    ) -> Self {
        Self(r.into(), g.into(), b.into(), a.into())
    }

    pub fn from_rgb(r: impl Into<u8>, g: impl Into<u8>, b: impl Into<u8>) -> Self {
        Self(r.into(), g.into(), b.into(), 255)
    }
    pub fn random_rgb() -> Self {
        Self(
            shared::random::get_inc(0u8, 255u8),
            shared::random::get_inc(0u8, 255u8),
            shared::random::get_inc(0u8, 255u8),
            255,
        )
    }
    pub fn random_rgba() -> Self {
        Self(
            shared::random::get_inc(0u8, 255u8),
            shared::random::get_inc(0u8, 255u8),
            shared::random::get_inc(0u8, 255u8),
            shared::random::get_inc(0u8, 255u8),
        )
    }

    pub fn rgba(&self) -> [u8; 4] {
        [self.red(), self.green(), self.blue(), self.alpha()]
    }
    pub fn red(&self) -> u8 {
        self.0
    }
    pub fn green(&self) -> u8 {
        self.1
    }
    pub fn blue(&self) -> u8 {
        self.2
    }
    pub fn alpha(&self) -> u8 {
        self.3
    }
    pub fn r(&self) -> u8 {
        self.red()
    }
    pub fn g(&self) -> u8 {
        self.green()
    }
    pub fn b(&self) -> u8 {
        self.blue()
    }
    pub fn a(&self) -> u8 {
        self.alpha()
    }
}

impl From<ggez::graphics::Color> for Color {
    fn from(ggezcolor: ggez::graphics::Color) -> Color {
        let (r, g, b, a) = ggezcolor.to_rgba();
        Color(r, g, b, a)
    }
}

impl From<Color> for ggez::graphics::Color {
    fn from(color: Color) -> ggez::graphics::Color {
        ggez::graphics::Color::from_rgba(color.0, color.1, color.2, color.3)
    }
}

impl From<[u8; 4]> for Color {
    fn from(u8array: [u8; 4]) -> Color {
        Color::from_rgba(u8array[0], u8array[1], u8array[2], u8array[3])
    }
}
