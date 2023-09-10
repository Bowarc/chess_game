// use {
//     crate::{assets, maths, render, utils::time},
//     ggez::{
//         graphics::{self, Drawable},
//         Context, GameResult,
//     },
// };
pub struct FrameStats {
    update_time: shared::time::Stopwatch,
    draw_time: shared::time::Stopwatch,
    frame_time: shared::time::Stopwatch,
    render_log: crate::render::RenderLog,
    // particles_log: (usize, usize), // number of sources, total number of particles
}

impl FrameStats {
    pub fn new() -> Self {
        Self {
            update_time: shared::time::Stopwatch::new(),
            draw_time: shared::time::Stopwatch::new(),
            frame_time: shared::time::Stopwatch::new(),
            render_log: crate::render::RenderLog::new(),
        }
    }
    pub fn begin_frame(&mut self) {
        self.frame_time.start()
    }
    pub fn begin_update(&mut self) {
        self.update_time.start()
    }
    pub fn begin_draw(&mut self) {
        self.draw_time.start()
    }
    pub fn end_frame(&mut self) {
        self.frame_time.stop()
    }
    pub fn end_update(&mut self) {
        self.update_time.stop()
    }
    pub fn end_draw(&mut self) {
        self.draw_time.stop()
    }
    pub fn frame_time(&self) -> std::time::Duration {
        self.frame_time.read()
    }
    pub fn update_time(&self) -> std::time::Duration {
        self.update_time.read()
    }
    pub fn draw_time(&self) -> std::time::Duration {
        self.draw_time.read()
    }
    pub fn set_render_log(&mut self, render_log: crate::render::RenderLog) {
        self.render_log = render_log
    }
    pub fn render_log(&self) -> crate::render::RenderLog {
        self.render_log
    }

    pub fn draw(
        &self,
        position: shared::maths::Point,
        ctx: &mut ggez::Context,
        render_request: &mut crate::render::RenderRequest,
        in_loading_requests: &[crate::assets::loader::Request],
    ) -> ggez::GameResult {
        use ggez::graphics::Drawable as _;

        let spacing = " ";
        let background_min_width = 272.;

        let time_frag = ggez::graphics::TextFragment::new(format!(
            "Time mesurements:\n{spacing}Fps        : {:.2}\n{spacing}Frame time : {}\n{spacing}Update time: {}\n{spacing}Draw time  : {}\n",
            // 1./ctx.time.delta().as_secs_f64(),
            ctx.time.fps(), // ctx.time.fps(), the first one is updating A LOT but is accurate, the latter is averaged over last 100 frames
            shared::time::display_duration(self.frame_time(), ""),
            shared::time::display_duration(self.update_time(), ""),
            shared::time::display_duration(self.draw_time(), ""),
        ))
        .color(ggez::graphics::Color::from_rgb(0, 150, 150));

        let mut asset_loading_debug_text = vec![];

        for req in in_loading_requests {
            asset_loading_debug_text.push(format!("{:?}", req))
        }
        let asset_loading_debug_text =
            format!("{asset_loading_debug_text:#?}").replace([',', '[', ']', '"'], "");

        let render_frag = ggez::graphics::TextFragment::new(format!(
            "Render:\n{spacing}Elements  : {}\n{spacing}Sprites   : {}\n{spacing}Sprite not found: {}\n{spacing}Meshes    : {}\n{spacing}Texts     : {}\n{spacing}Draw calls: {}\n{spacing}In loading assets: {}",
            self.render_log.elements(),
            self.render_log.sprites(),
            self.render_log.sprites_not_found(),
            self.render_log.meshes(),
            self.render_log.texts(),
            self.render_log.draw_calls(),
            asset_loading_debug_text,
        )).color(ggez::graphics::Color::from_rgb(150,150,0));

        let mut total_text = ggez::graphics::Text::new(time_frag);
        total_text.add(render_frag);

        total_text.set_layout(ggez::graphics::TextLayout::top_left());

        let ttd = total_text.dimensions(ctx).unwrap();
        render_request.add(
            total_text,
            crate::render::DrawParam::new().pos(position),
            crate::render::Layer::Ui,
        );

        render_request.add(
            ggez::graphics::Mesh::new_rectangle(
                ctx,
                ggez::graphics::DrawMode::fill(),
                shared::maths::Rect::new(
                    shared::maths::Point::new(ttd.x as f64, ttd.y as f64),
                    shared::maths::Vec2::new(ttd.w.max(background_min_width) as f64, ttd.h as f64),
                    0.,
                )
                .into(),
                ggez::graphics::Color::from_rgba(0, 0, 0, 200),
            )?,
            crate::render::DrawParam::default(),
            crate::render::Layer::UiBackground,
        );
        Ok(())
    }
}
