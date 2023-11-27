pub mod animation;
pub mod architecture;
pub mod pause_menu;

pub struct Gui {
    backend: ggegui::EguiBackend,
    pause_menu: pause_menu::PauseMenu,
}

impl Gui {
    pub fn new(
        // ctx: &mut impl ggez::context::Has<ggez::graphics::GraphicsContext>,
        ctx: &mut ggez::Context,
        cfg: &mut crate::config::Config,
    ) -> ggez::GameResult<Self> {
        let mut backend = ggegui::EguiBackend::new(ctx);
        backend
            .input
            .set_scale_factor(cfg.gui.scale as f32, ctx.gfx.size());
        Ok(Gui {
            backend,
            pause_menu: pause_menu::PauseMenu::new(ctx, cfg)?,
        })
    }

    pub fn is_visible(&self) -> bool {
        self.pause_menu.is_active()
    }

    pub fn backend_mut(&mut self) -> &mut ggegui::EguiBackend {
        &mut self.backend
    }

    pub fn update(
        &mut self,
        ggez_ctx: &mut ggez::Context,
        global_config: &mut crate::config::Config,
    ) -> ggez::GameResult {
        let egui_ctx = self.backend.ctx();
        self.pause_menu.update(ggez_ctx, &egui_ctx, global_config);
        self.backend.update(ggez_ctx);

        Ok(())
    }

    pub fn draw(
        &mut self,
        _ctx: &mut ggez::Context,
        render_request: &mut crate::render::RenderRequest,
    ) -> ggez::GameResult {
        render_request.add(
            crate::render::RenderRequestBit::EguiWindow,
            crate::render::DrawParam::new(),
            crate::render::Layer::UiForeground,
        );
        // canvas.draw(&self.backend, ggez::graphics::DrawParam::default())
        // ggez::graphics::draw(ctx, &self.backend, ([draw_offset.x, draw_offset.y],))?;

        Ok(())
    }
}
