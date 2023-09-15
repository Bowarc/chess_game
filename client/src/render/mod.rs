mod color;
mod draw_param;
mod layer;
mod render_log;
mod render_request;

pub use color::Color;
pub use draw_param::DrawParam;
pub use layer::Layer;
pub use render_log::RenderLog;
pub use render_request::{RenderRequest, RenderRequestBit};

pub struct Renderer {
    render_request: RenderRequest,
}

// currently the renderer has a 1 frame delay on the framestats render
// as for some obvious reasons, i can't display the stats of the frame as it's being drawn
// is it a problem ?
// i don't think so but anyway i have an idea of how to fix it if it becomes a problem

impl Renderer {
    pub fn new() -> Self {
        Self {
            render_request: RenderRequest::new(),
        }
    }

    pub fn render_request(&mut self) -> &mut RenderRequest {
        &mut self.render_request
    }

    // Sprite rendering is not done atm
    pub fn run(
        &mut self,
        ctx: &mut ggez::Context,
        menu_backend: &mut ggegui::EguiBackend,
    ) -> ggez::GameResult<RenderLog> {
        let mut layer_index = 0;
        let mut global_log = RenderLog::new();

        while let Some(layer) = Layer::get(layer_index) {
            layer_index += 1;
            let Some(bits) = self.render_request.get_mut(&layer)else{
                continue;
            };

            let mut canvas = ggez::graphics::Canvas::from_frame(ctx, None);

            global_log += Self::_run(ctx, &mut canvas, bits, menu_backend);

            canvas.finish(ctx)?;
        }

        self.render_request.clear();

        Ok(global_log)
    }
    fn _run(
        ctx: &mut ggez::Context,
        canvas: &mut ggez::graphics::Canvas,
        bits: &mut [(render_request::RenderRequestBit, DrawParam)],
        menu_backend: &mut ggegui::EguiBackend,
    ) -> RenderLog {
        // 'log' is already taken by the log crate, fuck you
        let mut log = RenderLog::new();

        let mut sprites_used = Vec::<crate::assets::sprite::SpriteId>::new();

        for (bit, dp) in bits {
            match bit {
                RenderRequestBit::Sprite(id) => {
                    todo!();
                    log.on_sprite();
                    sprites_used.push(*id)
                }
                RenderRequestBit::Mesh(mesh) => {
                    canvas.draw(mesh, dp.to_ggez_unscaled());
                    log.on_mesh();
                    log.on_draw_call();
                }
                RenderRequestBit::MeshBuilder(mesh_buuilder) => {
                    canvas.draw(
                        &ggez::graphics::Mesh::from_data(ctx, mesh_buuilder.build()),
                        dp.to_ggez_unscaled(),
                    );
                    log.on_mesh();
                    log.on_draw_call();
                }
                RenderRequestBit::Text(text) => {
                    canvas.draw(text, dp.to_ggez_unscaled());
                    log.on_text();
                    log.on_draw_call();
                }
                RenderRequestBit::EguiWindow => {
                    // In the ggegui implementation, the drawparam is discarded
                    canvas.draw(menu_backend, dp.to_ggez_unscaled());
                    log.on_draw_call();
                }
            }
        }

        log
    }
}
