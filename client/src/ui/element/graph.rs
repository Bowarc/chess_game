pub struct Graph {
    base: super::ElementBase,
}

impl Graph {
    pub fn new(base: super::ElementBase) -> Self {
        Self { base }
    }
}

impl super::TElement for Graph {
    fn get_base(&self) -> &crate::ui::element::ElementBase {
        &self.base
    }
    fn get_base_mut(&mut self) -> &mut crate::ui::element::ElementBase {
        &mut self.base
    }
    fn draw(
        &mut self,
        ctx: &mut ggez::Context,
        back: &mut ggez::graphics::MeshBuilder,
        ui: &mut ggez::graphics::MeshBuilder,
        front: &mut ggez::graphics::MeshBuilder,
        rr: &mut crate::render::RenderRequest,
    ) -> ggez::GameResult {
        error!("Graph draw function is not done yet");
        Ok(())
    }
}
