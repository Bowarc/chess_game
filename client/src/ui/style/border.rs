#[derive(Debug, Clone, Copy)]
pub struct Border {
    color: crate::render::Color,
    size: f64,
}
impl Border {
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

impl Default for Border {
    fn default() -> Self {
        Border {
            size: 5.,
            color: crate::render::Color::from_rgb(255, 0, 0),
        }
    }
}
