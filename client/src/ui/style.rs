mod background;
mod border;
mod bundle;

pub use background::*;
pub use border::*;
pub use bundle::*;

#[derive(Debug, Clone, Copy)]
pub struct Style {
    color: crate::render::Color,
    bg: Option<Background>,
    border: Option<Border>,
}

impl Style {
    pub fn new(
        color: crate::render::Color,
        bg: Option<Background>,
        border: Option<Border>,
    ) -> Self {
        Self { color, bg, border }
    }

    pub fn get_color_mut(&mut self) -> &mut crate::render::Color {
        &mut self.color
    }

    pub fn get_color(&self) -> &crate::render::Color {
        &self.color
    }

    pub fn get_bg_mut(&mut self) -> Option<&mut Background> {
        self.bg.as_mut()
    }
    pub fn get_bg(&self) -> Option<&Background> {
        self.bg.as_ref()
    }

    pub fn get_border_mut(&mut self) -> Option<&mut Border> {
        self.border.as_mut()
    }
    pub fn get_border(&self) -> Option<&Border> {
        self.border.as_ref()
    }
}

impl From<Style> for Bundle {
    fn from(value: Style) -> Self {
        Self::new(value, None, None)
    }
}

impl Default for Style {
    fn default() -> Self {
        Style {
            color: crate::render::Color::from_rgb(200, 200, 200),
            bg: None,
            border: Some(Border::default()),
        }
    }
}
