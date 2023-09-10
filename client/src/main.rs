#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use]
extern crate log;

mod assets;
mod config;
mod game;
mod networking;
mod render;
mod utils;

struct Chess {
    cfg: config::Config,
    renderer: render::Renderer,
    asset_mgr: assets::AssetManager,
    frame_stats: utils::framestats::FrameStats,
    client: networking::Client,
}

impl Chess {
    fn new(ctx: &mut ggez::Context, cfg: config::Config) -> ggez::GameResult<Self> {
        let mut client = networking::Client::new(shared::networking::DEFAULT_ADDRESS);
        client.request_ping().unwrap();
        let renderer = render::Renderer::new();

        let asset_mgr = assets::AssetManager::new();

        Ok(Self {
            cfg,
            renderer,
            asset_mgr,
            frame_stats: utils::framestats::FrameStats::new(),
            client,
        })
    }
}

impl ggez::event::EventHandler for Chess {
    /// Called upon each logic update to the game.
    /// This should be where the game's logic takes place.
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        self.frame_stats.end_frame();
        self.frame_stats.begin_frame();
        self.frame_stats.begin_update();

        let dt: f64 = ctx.time.delta().as_secs_f64();

        self.client.update();

        // self.gui_menu
        //     .update(ctx, &mut self.config, &self.game.entities.world)?;

        // self.assets.update(ctx, &self.config, &self.game);

        self.frame_stats.end_update();
        Ok(())
    }

    /// Called to do the drawing of your game.
    /// You probably want to start this with
    /// with [`graphics::present()`](../graphics/fn.present.html) and
    /// maybe [`timer::yield_now()`](../timer/fn.yield_now.html).
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        self.frame_stats.begin_draw();
        let dt: f64 = ctx.time.delta().as_secs_f64();
        let window_size: shared::maths::Vec2 = ctx.gfx.drawable_size().into();
        ggez::graphics::Canvas::from_frame(ctx, Some(render::Color::BLACK.into())).finish(ctx)?;

        let render_request = self.renderer.render_request();

        self.frame_stats.draw(
            shared::maths::Point::ZERO,
            ctx,
            render_request,
            self.asset_mgr.loader().ongoing_requests(),
        )?;

        let mesh = ggez::graphics::Mesh::new_circle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            shared::maths::Point::new(window_size.x / 2., window_size.y / 2.),
            100.,
            0.1,
            render::Color::WHITE.into(),
        )?;

        render_request.add(mesh, render::DrawParam::new(), render::Layer::Game);

        let render_log = self.renderer.run(ctx)?;

        self.frame_stats.set_render_log(render_log);

        self.frame_stats.end_draw();

        Ok(())
        // Err(ggez::error::GameError::CustomError("This is a test".into()))
    }

    /// A mouse button was pressed
    fn mouse_button_down_event(
        &mut self,
        ctx: &mut ggez::Context,
        button: ggez::input::mouse::MouseButton,
        _x: f32,
        _y: f32,
    ) -> std::result::Result<(), ggez::GameError> {
        // self.client.request_ping();
        Ok(())
    }

    /// A mouse button was released
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut ggez::Context,
        _button: ggez::input::mouse::MouseButton,
        _x: f32,
        _y: f32,
    ) -> std::result::Result<(), ggez::GameError> {
        Ok(())
    }

    /// The mouse was moved; it provides both absolute x and y coordinates in the window,
    /// and relative x and y coordinates compared to its last position.
    fn mouse_motion_event(
        &mut self,
        _ctx: &mut ggez::Context,
        _x: f32,
        _y: f32,
        _dx: f32,
        _dy: f32,
    ) -> std::result::Result<(), ggez::GameError> {
        Ok(())
    }

    /// mouse entered or left window area
    fn mouse_enter_or_leave(
        &mut self,
        _ctx: &mut ggez::Context,
        _entered: bool,
    ) -> std::result::Result<(), ggez::GameError> {
        Ok(())
    }

    /// The mousewheel was scrolled, vertically (y, positive away from and negative toward the user)
    /// or horizontally (x, positive to the right and negative to the left).
    fn mouse_wheel_event(
        &mut self,
        _ctx: &mut ggez::Context,
        x: f32,
        y: f32,
    ) -> std::result::Result<(), ggez::GameError> {
        // self.gui_menu
        //     .backend
        //     .input
        //     .mouse_wheel_event(x * 10., y * 10.);
        Ok(())
    }

    /// A keyboard button was pressed.
    ///
    /// The default implementation of this will call [`ctx.request_quit()`](crate::ggez::Context::request_quit)
    /// when the escape key is pressed. If you override this with your own
    /// event handler you have to re-implement that functionality yourself.
    fn key_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        _input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        Ok(())
    }

    /// A keyboard button was released.
    fn key_up_event(
        &mut self,
        _ctx: &mut ggez::Context,
        _input: ggez::input::keyboard::KeyInput,
    ) -> Result<(), ggez::GameError> {
        Ok(())
    }

    /// A unicode character was received, usually from keyboard input.
    /// This is the intended way of facilitating text input.
    fn text_input_event(
        &mut self,
        _ctx: &mut ggez::Context,
        character: char,
    ) -> std::result::Result<(), ggez::GameError> {
        // self.gui_menu.backend.input.text_input_event(character);
        Ok(())
    }

    /// An event from a touchscreen has been triggered; it provides the x and y location
    /// inside the window as well as the state of the tap (such as Started, Moved, Ended, etc)
    /// By default, touch events will trigger mouse behavior
    fn touch_event(
        &mut self,
        _ctx: &mut ggez::Context,
        _phase: ggez::event::winit_event::TouchPhase,
        _x: f64,
        _y: f64,
    ) -> Result<(), ggez::GameError> {
        Ok(())
    }

    /// A gamepad button was pressed; `id` identifies which gamepad.
    /// Use [`input::gamepad()`](../input/fn.gamepad.html) to get more info about
    /// the gamepad.
    fn gamepad_button_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        _btn: ggez::event::Button,
        _id: ggez::input::gamepad::GamepadId,
    ) -> std::result::Result<(), ggez::GameError> {
        Ok(())
    }

    /// A gamepad button was released; `id` identifies which gamepad.
    /// Use [`input::gamepad()`](../input/fn.gamepad.html) to get more info about
    /// the gamepad.
    fn gamepad_button_up_event(
        &mut self,
        _ctx: &mut ggez::Context,
        _btn: ggez::event::Button,
        _id: ggez::input::gamepad::GamepadId,
    ) -> std::result::Result<(), ggez::GameError> {
        Ok(())
    }

    /// A gamepad axis moved; `id` identifies which gamepad.
    /// Use [`input::gamepad()`](../input/fn.gamepad.html) to get more info about
    /// the gamepad.
    fn gamepad_axis_event(
        &mut self,
        _ctx: &mut ggez::Context,
        _axis: ggez::event::Axis,
        _value: f32,
        _id: ggez::input::gamepad::GamepadId,
    ) -> std::result::Result<(), ggez::GameError> {
        Ok(())
    }

    /// Called when the window is shown or hidden.
    fn focus_event(
        &mut self,
        _ctx: &mut ggez::Context,
        _gained: bool,
    ) -> std::result::Result<(), ggez::GameError> {
        Ok(())
    }

    /// Called upon a quit event.  If it returns true,
    /// the game does not exit (the quit event is cancelled).
    fn quit_event(
        &mut self,
        _ctx: &mut ggez::Context,
    ) -> std::result::Result<bool, ggez::GameError> {
        debug!("See you next time. . .");
        // self.client.shutdown();

        spin_sleep::sleep(std::time::Duration::from_millis(100));

        Ok(false)
    }

    /// Called when the user resizes the window, or when it is resized
    /// via [`graphics::set_mode()`](../graphics/fn.set_mode.html).
    fn resize_event(
        &mut self,
        _ctx: &mut ggez::Context,
        _width: f32,
        _height: f32,
    ) -> std::result::Result<(), ggez::GameError> {
        Ok(())
    }

    /// Something went wrong, causing a `GameError`.
    /// If this returns true, the error was fatal, so the event loop ends, aborting the game.
    fn on_error(
        &mut self,
        _ctx: &mut ggez::Context,
        _origin: ggez::event::ErrorOrigin,
        e: ggez::GameError,
    ) -> bool {
        error!("{e}");
        // for testing
        error!("Unexpected error, exiting...");
        true
    }
}

fn main() -> ggez::GameResult {
    let logger_config = shared::logger::LoggerConfig::new()
        .set_level(log::LevelFilter::Trace)
        .add_filter("wgpu_core", log::LevelFilter::Warn)
        .add_filter("wgpu_hal", log::LevelFilter::Error)
        .add_filter("naga", log::LevelFilter::Warn);
    shared::logger::init(logger_config, Some("Client.log"));
    shared::logger::test();

    debug!("Testing!!!");

    let config: config::Config = config::load();

    let cb = ggez::ContextBuilder::new("Chess game", "Bowarc")
        .resources_dir_name("resources\\external\\")
        .window_setup(
            ggez::conf::WindowSetup::default()
                .title("Vupa")
                .samples(config.window.samples)
                .vsync(config.window.v_sync)
                // .icon("/icon/logoV1.png")// .icon(iconpath.as_str()), // .icon("/Python.ico"),
                .srgb(config.window.srgb),
        )
        .window_mode(config.window.into())
        .backend(ggez::conf::Backend::default());

    // if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
    //     let mut path = std::path::PathBuf::from(manifest_dir);
    //     path.push("resources");
    //     debug!("Adding path {:?}", path);
    //     cb = cb.add_resource_path(path);
    // }

    let (mut ctx, event_loop) = cb.build()?;

    let game = Chess::new(&mut ctx, config)?;

    ggez::event::run(ctx, event_loop, game);
}
