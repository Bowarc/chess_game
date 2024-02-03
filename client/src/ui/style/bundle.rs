use super::Style;

/// idk i keep this here, only the button will use this
#[derive(Default, Debug, Clone, Copy)]
pub struct Bundle {
    default: Style,
    hovered: Option<Style>,
    clicked: Option<Style>,
}

impl Bundle {
    pub fn get(&self, state: &crate::ui::State) -> Style {
        if state.clicked() {
            self.clicked.unwrap_or(self.default)
        } else if state.hovered() {
            self.hovered.unwrap_or(self.default)
        } else {
            self.default
        }
    }
}

impl Bundle {
    pub fn new(default: Style, hovered: Option<Style>, clicked: Option<Style>) -> Self {
        Self {
            default,
            hovered,
            clicked,
        }
    }
    pub fn get_default_mut(&mut self) -> &mut Style {
        &mut self.default
    }
    pub fn get_default(&self) -> &Style {
        &self.default
    }

    pub fn get_hovered_mut(&mut self) -> Option<&mut Style> {
        self.hovered.as_mut()
    }
    pub fn get_hovered(&self) -> Option<&Style> {
        self.hovered.as_ref()
    }

    pub fn get_clicked_mut(&mut self) -> Option<&mut Style> {
        self.clicked.as_mut()
    }
    pub fn get_clicked(&self) -> Option<&Style> {
        self.clicked.as_ref()
    }
}
