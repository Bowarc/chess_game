pub struct Button {
    base: super::ElementBase,
}

impl Button {
    pub fn new(base: super::ElementBase) -> Self {
        Self { base }
    }
}

impl super::TElement for Button {
    fn get_base(&self) -> &crate::ui::element::ElementBase {
        &self.base
    }
    fn get_base_mut(&mut self) -> &mut crate::ui::element::ElementBase {
        &mut self.base
    }
    fn draw(
        &mut self,
        ctx: &mut ggez::Context,
        back_mesh: &mut ggez::graphics::MeshBuilder,
        ui_mesh: &mut ggez::graphics::MeshBuilder,
        front_mesh: &mut ggez::graphics::MeshBuilder,
        rr: &mut crate::render::RenderRequest,
    ) -> ggez::GameResult {
        let rect = self.get_computed_rect(ctx);
        let style = self.base.style.get(&self.base.state);

        if let Some(border) = style.get_border() {
            let r = shared::maths::Rect::new(
                rect.r_topleft() - border.get_size() / 2.,
                rect.size() + *border.get_size(),
                rect.rotation(),
            );

            front_mesh.rectangle(
                ggez::graphics::DrawMode::stroke(*border.get_size() as f32),
                r.into(),
                (*border.get_color()).into(),
            )?;
        };

        ui_mesh.rectangle(
            ggez::graphics::DrawMode::fill(),
            rect.into(),
            (*style.get_color()).into(),
        )?;

        Ok(())
    }
}
