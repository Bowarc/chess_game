const SPACING: &str = " ";
const BACKGROUND_MIN_WIDTH: f32 = 272.;

pub struct FrameStats {
    update_time: time::Stopwatch,
    draw_time: time::Stopwatch,
    frame_time: time::Stopwatch,
    render_log: crate::render::RenderLog,
}

impl FrameStats {
    pub fn new() -> Self {
        Self {
            update_time: time::Stopwatch::new(),
            draw_time: time::Stopwatch::new(),
            frame_time: time::Stopwatch::new(),
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
        network_stats_opt: Option<
            &networking::NetworkStats<
                shared::message::ServerMessage,
                shared::message::ClientMessage,
            >,
        >,
    ) -> ggez::GameResult {
        use ggez::graphics::Drawable as _;

        let time_frag = self.draw_time_measurements(ctx);
        let render_frag = self.draw_render_stats(in_loading_requests);

        let mut total_text = ggez::graphics::Text::new(time_frag);
        total_text.add(render_frag);

        if let Some(network_stats) = network_stats_opt {
            let network_frag = self.draw_network(network_stats);
            total_text.add(network_frag);
        }

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
                    shared::maths::Vec2::new(ttd.w.max(BACKGROUND_MIN_WIDTH) as f64, ttd.h as f64),
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

    fn draw_render_stats(
        &self,
        in_loading_requests: &[crate::assets::loader::Request],
    ) -> ggez::graphics::TextFragment {
        let mut asset_loading_debug_text = vec![];

        for req in in_loading_requests {
            asset_loading_debug_text.push(format!("{:?}", req))
        }
        let asset_loading_debug_text =
            format!("{asset_loading_debug_text:#?}").replace([',', '[', ']', '"'], "");

        ggez::graphics::TextFragment::new(format!(
            "Render:\n{SPACING}Elements  : {elements}\n{SPACING}Sprites   : {sprites}\n{SPACING}Sprite not found: {sprites_not_found}\n{SPACING}Meshes    : {meshes}\n{SPACING}Texts     : {texts}\n{SPACING}Draw calls: {draw_calls}\n{SPACING}In loading assets: {in_loading_assets}\n",
            elements = self.render_log.elements(),
            sprites = self.render_log.sprites(),
            sprites_not_found = self.render_log.sprites_not_found(),
            meshes = self.render_log.meshes(),
            texts= self.render_log.texts(),
            draw_calls= self.render_log.draw_calls(),
            in_loading_assets= asset_loading_debug_text,
        )).color(ggez::graphics::Color::from_rgb(150,150,0))
    }

    fn draw_time_measurements(&self, ctx: &ggez::Context) -> ggez::graphics::TextFragment {
        ggez::graphics::TextFragment::new(format!(
            "Time mesurements:\n{SPACING}Fps        : {fps:.2}\n{SPACING}Frame time : {frame_time}\n{SPACING}Update time: {update_time}\n{SPACING}Draw time  : {draw_time}\n",
            // 1./ctx.time.delta().as_secs_f64(),
            fps = ctx.time.fps(), // ctx.time.fps(), the first one is updating A LOT but is accurate, the latter is averaged over last 100 frames
            frame_time = time::format(self.frame_time(), 1),
            update_time = time::format(self.update_time(), 1),
            draw_time = time::format(self.draw_time(), 1),
        ))
        .color(ggez::graphics::Color::from_rgb(0, 150, 150))
    }

    fn draw_network(
        &self,
        network_stats: &networking::NetworkStats<
            shared::message::ServerMessage,
            shared::message::ClientMessage,
        >,
    ) -> ggez::graphics::TextFragment {
        ggez::graphics::TextFragment::new(format!(
            "Networking:\n{SPACING}RTT: {rtt}\n{SPACING}I/O: {i}/{o}\n{SPACING}I/O (10s): {i10s}/{o10s}\n{SPACING}IOPS {ips}/{ops}",
            rtt = time::format(network_stats.get_rtt(), 1),
            i = mem::display_bytes(network_stats.total_received()),
            o = mem::display_bytes(network_stats.total_sent()),
            i10s = mem::display_bytes(network_stats.received_last_10_sec()),
            o10s = mem::display_bytes(network_stats.sent_last_10_sec()),
            ips = mem::display_bytes(network_stats.bps_received_last_10_sec()),
            ops = mem::display_bytes(network_stats.bps_sent_last_10_sec()),
        ))
        .color(ggez::graphics::Color::from_rgb(0, 150, 0))
    }
}
