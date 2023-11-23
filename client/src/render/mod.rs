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
        loader_handle: &mut crate::assets::loader::Handle,
        sprite_bank: &mut impl crate::assets::Bank<
            crate::assets::sprite::SpriteId,
            ggez::graphics::InstanceArray,
        >,
    ) -> ggez::GameResult<RenderLog> {
        let mut layer_index = 0;
        let mut global_log = RenderLog::new();

        while let Some(layer) = Layer::get(layer_index) {
            layer_index += 1;
            let Some(bits) = self.render_request.get_mut(&layer) else {
                continue;
            };

            let mut canvas = ggez::graphics::Canvas::from_frame(ctx, None);

            global_log += Self::_run(
                ctx,
                &mut canvas,
                bits,
                menu_backend,
                loader_handle,
                sprite_bank,
            );

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
        loader_handle: &mut crate::assets::loader::Handle,
        sprite_bank: &mut impl crate::assets::Bank<
            crate::assets::sprite::SpriteId,
            ggez::graphics::InstanceArray,
        >,
    ) -> RenderLog {
        use ggez::graphics::Drawable as _;
        // 'log' is already taken by the log crate, fuck you
        let mut log = RenderLog::new();

        let mut sprites_used = Vec::<crate::assets::sprite::SpriteId>::new();

        for (bit, dp) in bits {
            match bit {
                RenderRequestBit::Sprite(id) => {
                    /*
                        Here appends something interesing,
                        If the requested id is not yet loaded, the default InstanceArray retrieved (auto by try_get_mut)

                        And then the id (that we fail to fetch) is sent to sprites_used (BUT WE DIDDN'T USED THAT InstanceArray)
                        Then, using the ids from sprites_used (that the renderer thinks it used) queries again (but faills)
                        so the default InstanceArray is retrieved and cleaned.

                        There is no sprite bank update between thoses queries, so it's working.

                        This could be fixed by 2 things
                        1) Return None when the given Id is not yet loaded,
                            But i like using default sprites for things that are not yet loaded.
                        2) Find a way for the renderer to know that the querry failled and the id isn't the right one.
                    */
                    let Some(ia) = sprite_bank.try_get_mut(id, loader_handle) else {
                        error!("Could not get instance array for sprite {id:?}");
                        continue;
                    };
                    let Some(dimensions) = ia.image().dimensions(ctx) else {
                        error!("Could not query the size of the image for sprite {id:?}");
                        continue;
                    };
                    ia.push(dp.to_ggez_scaled(dimensions.size()));

                    if !sprites_used.contains(id) {
                        sprites_used.push(*id)
                    }

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

        for id in sprites_used.iter() {
            // The implicit unwrap of get_mut is fine as any sprite in this list has been queried before so it *should* be loaded
            let ia = sprite_bank.get_mut(id, loader_handle);

            canvas.draw(ia, DrawParam::default());
            log.on_draw_call();
            ia.clear()
        }

        log
    }
}
