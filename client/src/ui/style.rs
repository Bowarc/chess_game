/// idk i keep this here, only the button will use this
#[derive(Default, Debug, Clone, Copy)]
pub struct Bundle {
    default: Style,
    hovered: Option<Style>,
    clicked: Option<Style>,
}
#[derive(Debug, Clone, Copy)]
pub struct Style {
    color: crate::render::Color,
    bg: Option<BackgroundStyle>,
    border: Option<BorderStyle>,
}
#[derive(Debug, Clone, Copy)]
pub struct BackgroundStyle {
    color: crate::render::Color,
    img: Option<crate::assets::sprite::SpriteId>,
}
#[derive(Debug, Clone, Copy)]
pub struct BorderStyle {
    color: crate::render::Color,
    size: f64,
}

impl Bundle {
    pub fn get(&self, state: &super::State) -> Style {
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

impl Style {
    pub fn new(
        color: crate::render::Color,
        bg: Option<BackgroundStyle>,
        border: Option<BorderStyle>,
    ) -> Self {
        Self { color, bg, border }
    }

    pub fn get_color_mut(&mut self) -> &mut crate::render::Color {
        &mut self.color
    }

    pub fn get_color(&self) -> &crate::render::Color {
        &self.color
    }

    pub fn get_bg_mut(&mut self) -> Option<&mut BackgroundStyle> {
        self.bg.as_mut()
    }
    pub fn get_bg(&self) -> Option<&BackgroundStyle> {
        self.bg.as_ref()
    }

    pub fn get_border_mut(&mut self) -> Option<&mut BorderStyle> {
        self.border.as_mut()
    }
    pub fn get_border(&self) -> Option<&BorderStyle> {
        self.border.as_ref()
    }
}

impl BackgroundStyle {
    pub fn new(color: crate::render::Color, img: Option<crate::assets::sprite::SpriteId>) -> Self {
        Self { color, img }
    }

    pub fn get_color_mut(&mut self) -> &mut crate::render::Color {
        &mut self.color
    }

    pub fn get_color(&self) -> &crate::render::Color {
        &self.color
    }

    pub fn get_img_mut(&mut self) -> Option<&mut crate::assets::sprite::SpriteId> {
        self.img.as_mut()
    }

    pub fn get_img(&self) -> Option<&crate::assets::sprite::SpriteId> {
        self.img.as_ref()
    }

    pub fn draw(
        &self,
        mesh: &mut ggez::graphics::MeshBuilder,
        render_request: &mut crate::render::RenderRequest,
        element_rect: shared::maths::Rect,
    ) -> ggez::GameResult {
        if let Some(sprite_id) = self.get_img() {
            render_request.add(
                *sprite_id,
                crate::render::DrawParam::default()
                    .pos(element_rect.aa_topleft())
                    .size(element_rect.size())
                    .color(*self.get_color()),
                crate::render::Layer::UiBackground,
            )
        } else {
            mesh.rectangle(
                ggez::graphics::DrawMode::fill(),
                element_rect.into(),
                (*self.get_color()).into(),
            )?;
        }

        Ok(())
    }
}

impl BorderStyle {
    pub fn new(color: crate::render::Color, size: f64) -> Self {
        Self { color, size }
    }

    pub fn get_color_mut(&mut self) -> &mut crate::render::Color {
        &mut self.color
    }

    pub fn get_color(&self) -> &crate::render::Color {
        &self.color
    }
    pub fn get_size_mut(&mut self) -> &mut f64 {
        &mut self.size
    }
    pub fn get_size(&self) -> &f64 {
        &self.size
    }

    pub fn draw(
        &self,
        mesh: &mut ggez::graphics::MeshBuilder,
        element_rect: shared::maths::Rect,
    ) -> ggez::GameResult {
        let r = shared::maths::Rect::new(
            element_rect.r_topleft() - self.get_size() * 0.5,
            element_rect.size() + *self.get_size(),
            element_rect.rotation(),
        );

        mesh.rectangle(
            ggez::graphics::DrawMode::stroke(*self.get_size() as f32),
            r.into(),
            (*self.get_color()).into(),
        )?;
        Ok(())
    }
}

impl Default for Style {
    fn default() -> Self {
        Style {
            color: crate::render::Color::from_rgb(200, 200, 200),
            bg: None,
            border: Some(BorderStyle::default()),
        }
    }
}

impl Default for BackgroundStyle {
    fn default() -> Self {
        BackgroundStyle {
            color: crate::render::Color::from_rgb(24, 24, 24),
            img: None,
        }
    }
}

impl Default for BorderStyle {
    fn default() -> Self {
        BorderStyle {
            size: 5.,
            color: crate::render::Color::from_rgb(255, 0, 0),
        }
    }
}
