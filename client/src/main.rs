// #![allow(dead_code)]
// #![allow(unused_variables)]

#[macro_use]
extern crate log;

mod action;
mod assets;
mod config;
mod game;
mod gui;
mod input;
mod networking;
mod render;
mod ui;
mod utils;

struct Chess {
    cfg: config::Config,
    renderer: render::Renderer,
    asset_mgr: assets::AssetManager,
    frame_stats: utils::framestats::FrameStats,
    gui_menu: gui::Gui,
    global_ui: ui::UiManager,
    game_state: game::Game,
}

impl Chess {
    fn new(ctx: &mut ggez::Context, mut cfg: config::Config) -> ggez::GameResult<Self> {
        let renderer = render::Renderer::new();

        let gui_menu = gui::Gui::new(ctx, &mut cfg)?;

        let asset_mgr = assets::AssetManager::new();

        let mut global_ui = ui::UiManager::default();

        let id = global_ui.add_element(
            ui::element::Element::new_graph(
                "fps graph",
                (ui::Anchor::TopRight, (-2., 2.)),
                (200., 50.),
                ui::Style::new(
                    render::Color::random_rgb(),
                    Some(ui::style::BackgroundStyle::new(
                        render::Color::from_rgba(23, 23, 23, 150),
                        Some(assets::sprite::SpriteId::MissingNo),
                    )),
                    Some(ui::style::BorderStyle::new(render::Color::WHITE, 1.)),
                ),
                Some(
                    ui::element::GraphText::default()
                        .anchor(ui::Anchor::Topleft)
                        .offset(shared::maths::Vec2::ONE)
                        .text(|val| -> String { format!("{}fps", val as i32) })
                        .size(5.)
                        .color(render::Color::random_rgb()),
                ),
            ),
            "",
        );
        let id2 = global_ui.add_element(
            ui::element::Element::new_graph(
                "rtt graph",
                (ui::Anchor::TopRight, (-2., 52.)),
                (200., 50.),
                ui::Style::new(
                    render::Color::random_rgb(),
                    None,
                    Some(ui::style::BorderStyle::new(render::Color::WHITE, 1.)),
                ),
                Some(
                    ui::element::GraphText::default()
                        .anchor(ui::Anchor::Topleft)
                        .offset(shared::maths::Vec2::ONE)
                        .text(|val| -> String {
                            format!(
                                "RTT: {}",
                                time::display_duration(std::time::Duration::from_secs_f64(val))
                            )
                        })
                        .size(8.)
                        .color(render::Color::random_rgb()),
                ),
            ),
            "",
        );

        // let text_id = global_ui.add_element(ui::element::Element::new_text(
        //     ui::Position::new_anchor(ui::Anchor::TopCenter, (0., 2.)),
        //     20.,
        //     ui::Style::new(
        //         render::Color::WHITE,
        //         // Some(ui::style::BackgroundStyle::new(render::Color::WHITE, Some(assets::sprite::SpriteId::MissingNo))),
        //         None,
        //         Some(ui::style::BorderStyle::new(render::Color::random_rgb(), 2.)),
        //     ),
        //     vec![
        //         ui::element::TextBit::new_text(
        //             String::from("This is a test string"),
        //             Some(render::Color::from_rgb(255, 0, 0)),
        //         ),
        //         ui::element::TextBit::new_text(
        //             String::from("This is a test string2"),
        //             Some(render::Color::from_rgb(0, 255, 0)),
        //         ),
        //         ui::element::TextBit::new_text(
        //             String::from("This is a test string3"),
        //             Some(render::Color::from_rgb(0, 0, 255)),
        //         ),
        //         ui::element::TextBit::new_text(
        //             String::from("\n"),
        //             Some(render::Color::from_rgb(0, 255, 0)),
        //         ),
        //         ui::element::TextBit::new_img(assets::sprite::SpriteId::MissingNo),
        //         ui::element::TextBit::new_text(
        //             String::from("This seccond string should be on another line|"),
        //             Some(render::Color::from_rgb(0, 0, 255)),
        //         ),
        //         ui::element::TextBit::new_text("".to_string(), None),
        //         ui::element::TextBit::new_text(
        //             String::from("\n\nNew String\n"),
        //             Some(render::Color::random_rgb()),
        //         ),
        //         ui::element::TextBit::NewLine,
        //         ui::element::TextBit::new_img(assets::sprite::SpriteId::MissingNo)
        //     ],
        // ));

        // let text_edit_id = global_ui.add_element(ui::element::Element::new_text_edit(
        //     "Text edit 1",
        //     ui::Position::new_anchor(ui::Anchor::TopCenter, (0., 2.)),
        //     200.,
        //     3,
        //     40.,
        //     ui::style::Bundle::new(
        //         ui::Style::new(
        //             render::Color::default(),
        //             Some(ui::style::BackgroundStyle::new(
        //                 render::Color::random_rgb(),
        //                 None,
        //             )),
        //             Some(ui::style::BorderStyle::new(render::Color::random_rgb(), 1.)),
        //         ),
        //         Some(ui::Style::new(
        //             render::Color::random_rgb(),
        //             Some(ui::style::BackgroundStyle::new(
        //                 render::Color::random_rgb(),
        //                 None,
        //             )),
        //             Some(ui::style::BorderStyle::default()),
        //         )),
        //         Some(ui::Style::new(
        //             render::Color::random_rgb(),
        //             None,
        //             Some(ui::style::BorderStyle::new(render::Color::random_rgb(), 1.)),
        //         )),
        //     ),
        // ));

        let mp_id = global_ui.add_element(
            ui::element::Element::new_text(
                "mouse pos text",
                (ui::Anchor::BotRight, (-1., -1.)),
                20.,
                ui::Style::new(
                    render::Color::random_rgb(),
                    Some(ui::style::BackgroundStyle::new(
                        render::Color::from_rgba(20, 20, 20, 100),
                        None,
                    )),
                    Some(ui::style::BorderStyle::new(render::Color::random_rgb(), 1.)),
                ),
                vec![ui::element::TextBit::new_text("".to_string(), None)],
            ),
            "",
        );

        let test_t_id = global_ui.add_element(
            ui::element::Element::new_text(
                "Test_image_id",
                (ui::Anchor::TopCenter, (0., 100.)),
                40.,
                ui::Style::new(
                    render::Color::random_rgb(),
                    None,
                    Some(ui::style::BorderStyle::new(render::Color::random_rgb(), 1.)),
                ),
                vec![
                    ui::element::TextBit::new_text(
                        "15 ",
                        Some(render::Color::from_rgb(186, 80, 40)),
                    ),
                    ui::element::TextBit::new_text(
                        "(+1.1",
                        Some(render::Color::from_rgb(215, 148, 95)),
                    ),
                    ui::element::TextBit::new_img(
                        assets::sprite::SpriteId::AttackDamage,
                        Some(render::Color::from_rgb(215, 148, 95)),
                    ),
                    ui::element::TextBit::new_text(
                        ")",
                        Some(render::Color::from_rgb(215, 148, 95)),
                    ),
                    ui::element::TextBit::new_text(
                        " (+0.4",
                        Some(render::Color::from_rgb(105, 255, 249)),
                    ),
                    ui::element::TextBit::new_img(
                        assets::sprite::SpriteId::AbilityPower,
                        Some(render::Color::from_rgb(105, 255, 249)),
                    ),
                    ui::element::TextBit::new_text(
                        ")",
                        Some(render::Color::from_rgb(105, 255, 249)),
                    ),
                ],
            ),
            "",
        );

        {
            let anchors = [
                ui::Anchor::CenterCenter,
                ui::Anchor::Topleft,
                ui::Anchor::TopCenter,
                ui::Anchor::TopRight,
                ui::Anchor::RightCenter,
                ui::Anchor::BotRight,
                ui::Anchor::BotCenter,
                ui::Anchor::BotLeft,
                ui::Anchor::LeftCenter,
            ];

            for anchor in anchors.iter() {
                let new_game_button = ui::element::Element::new_button(
                    format!("test{anchor:?}"),
                    *anchor,
                    ui::Vector::new(50., 50.),
                    ui::Style::new(
                        render::Color::from_rgba(100, 100, 100, 100),
                        Some(ui::style::BackgroundStyle::new(
                            render::Color::from_rgb(100, 100, 100),
                            None,
                        )),
                        Some(ui::style::BorderStyle::new(render::Color::random_rgb(), 2.)),
                    )
                    .into(),
                );
                global_ui.add_element(new_game_button, "test");
            }
        }

        Ok(Self {
            cfg,
            renderer,
            asset_mgr,
            frame_stats: utils::framestats::FrameStats::new(),
            gui_menu,
            global_ui,
            game_state: game::Game::new(),
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

        self.gui_menu.update(ctx, &mut self.cfg)?;

        self.game_state.update(ctx,dt);

        self.global_ui.update(ctx);

        // if self
        //     .global_ui
        //     .get_element("board square 0x0")
        //     .inner::<ui::element::Button>()
        //     .clicked_this_frame()
        // {
        //     debug!("Clicked this frame")
        // }

        self.global_ui
            .get_element("fps graph")
            .inner_mut::<ui::element::Graph>()
            .push(ctx.time.fps(), dt);

        self.global_ui
            .get_element("rtt graph")
            .inner_mut::<ui::element::Graph>()
            .push(
                self.game_state
                    .try_get_client_mut()
                    .map(|client| client.stats().get_rtt().as_secs_f64())
                    .unwrap_or(0.),
                dt,
            );

        self.global_ui
            .get_element("mouse pos text")
            .inner_mut::<ui::element::Text>()
            .replace_bits(vec![
                ui::element::TextBit::new_text(format!("{:?}", ctx.mouse.position()), None),
                ui::element::TextBit::new_img(assets::sprite::SpriteId::MissingNo, None),
            ]);

        self.asset_mgr.update(ctx);

        self.frame_stats.end_update();
        Ok(())
    }

    /// Called to do the drawing of your game.
    /// You probably want to start this with
    /// with [`graphics::present()`](../graphics/fn.present.html) and
    /// maybe [`timer::yield_now()`](../timer/fn.yield_now.html).
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        self.frame_stats.begin_draw();

        // let window_size: shared::maths::Vec2 = ctx.gfx.drawable_size().into();
        ggez::graphics::Canvas::from_frame(ctx, Some(render::Color::BLACK.into())).finish(ctx)?;

        let render_request = self.renderer.render_request();

        self.frame_stats.draw(
            shared::maths::Point::ZERO,
            ctx,
            render_request,
            self.asset_mgr.get_loader().ongoing_requests(),
            self.game_state.try_get_client_mut().and_then(|client| {
                if client.is_connected() {
                    Some(client.stats())
                } else {
                    None
                }
            }),
        )?;
        self.gui_menu.draw(ctx, render_request)?;

        if let Some(ui_mgr) = self.game_state.try_get_ui_mgr_mut() {
            ui_mgr.draw(ctx, render_request)?;
        }

        self.global_ui.draw(ctx, render_request)?;

        self.game_state.draw(render_request);
        let render_log = self.renderer.run(
            ctx,
            self.gui_menu.backend_mut(),
            &mut self.asset_mgr.loader_handle,
            &mut self.asset_mgr.sprite_bank,
        )?;

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
        x: f32,
        y: f32,
    ) -> ggez::GameResult {
        self.global_ui.register_mouse_press(button, x, y);
        if let Some(ui) = self.game_state.try_get_ui_mgr_mut() {
            ui.register_mouse_press(button, x, y)
        }

        Ok(())
    }

    /// A mouse button was released
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut ggez::Context,
        button: ggez::input::mouse::MouseButton,
        x: f32,
        y: f32,
    ) -> ggez::GameResult {
        self.global_ui.register_mouse_release(button, x, y);
        if let Some(ui) = self.game_state.try_get_ui_mgr_mut() {
            ui.register_mouse_release(button, x, y);
        }
        Ok(())
    }

    /// The mouse was moved; it provides both absolute x and y coordinates in the window,
    /// and relative x and y coordinates compared to its last position.
    fn mouse_motion_event(
        &mut self,
        _ctx: &mut ggez::Context,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
    ) -> ggez::GameResult {
        self.global_ui.register_mouse_motion(x, y, dx, dy);
        if let Some(ui) = self.game_state.try_get_ui_mgr_mut() {
            ui.register_mouse_motion(x, y, dx, dy)
        }
        Ok(())
    }

    /// mouse entered or left window area
    fn mouse_enter_or_leave(
        &mut self,
        _ctx: &mut ggez::Context,
        _entered: bool,
    ) -> ggez::GameResult {
        Ok(())
    }

    /// The mousewheel was scrolled, vertically (y, positive away from and negative toward the user)
    /// or horizontally (x, positive to the right and negative to the left).
    fn mouse_wheel_event(&mut self, _ctx: &mut ggez::Context, x: f32, y: f32) -> ggez::GameResult {
        self.gui_menu
            .backend_mut()
            .input
            .mouse_wheel_event(x * 10., y * 10.);
        self.global_ui.register_mouse_wheel(x, y);
        if let Some(ui) = self.game_state.try_get_ui_mgr_mut() {
            ui.register_mouse_wheel(x, y);
        }
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
        input: ggez::input::keyboard::KeyInput,
        repeated: bool,
    ) -> ggez::GameResult {
        self.global_ui.register_key_down(input, repeated);
        if let Some(ui) = self.game_state.try_get_ui_mgr_mut() {
            ui.register_key_down(input, repeated);
        }
        Ok(())
    }

    /// A keyboard button was released.
    fn key_up_event(
        &mut self,
        _ctx: &mut ggez::Context,
        input: ggez::input::keyboard::KeyInput,
    ) -> ggez::GameResult {
        self.global_ui.register_key_up(input);
        if let Some(ui) = self.game_state.try_get_ui_mgr_mut() {
            ui.register_key_up(input);
        }
        Ok(())
    }

    /// A unicode character was received, usually from keyboard input.
    /// This is the intended way of facilitating text input.
    fn text_input_event(&mut self, _ctx: &mut ggez::Context, character: char) -> ggez::GameResult {
        self.gui_menu
            .backend_mut()
            .input
            .text_input_event(character);
        self.global_ui.register_text_input(character);
        if let Some(ui) = self.game_state.try_get_ui_mgr_mut() {
            ui.register_text_input(character);
        }
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
    ) -> ggez::GameResult {
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
    ) -> ggez::GameResult {
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
    ) -> ggez::GameResult {
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
    ) -> ggez::GameResult {
        Ok(())
    }

    /// Called when the window is shown or hidden.
    fn focus_event(&mut self, _ctx: &mut ggez::Context, _gained: bool) -> ggez::GameResult {
        Ok(())
    }

    /// Called upon a quit event.  If it returns true,
    /// the game does not exit (the quit event is cancelled).
    fn quit_event(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult<bool> {
        debug!("See you next time. . .");

        Ok(false)
    }

    /// Called when the user resizes the window, or when it is resized
    /// via [`graphics::set_mode()`](../graphics/fn.set_mode.html).
    fn resize_event(
        &mut self,
        _ctx: &mut ggez::Context,
        _width: f32,
        _height: f32,
    ) -> ggez::GameResult {
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

        true
    }
}

fn main() -> ggez::GameResult {
    let logger_config = logger::LoggerConfig::new()
        .set_level(log::LevelFilter::Trace)
        .add_filter("wgpu_core", log::LevelFilter::Warn)
        .add_filter("wgpu_hal", log::LevelFilter::Error)
        .add_filter("gilrs", log::LevelFilter::Off)
        .add_filter("naga", log::LevelFilter::Warn)
        .add_filter("networking", log::LevelFilter::Debug)
        .add_filter("ggez", log::LevelFilter::Warn);
    logger::init(logger_config, Some("./log/client.log"));
    // logger::test();

    shared::file::list();

    let config: config::Config = config::load();

    let cb = ggez::ContextBuilder::new("Chess game", "Bowarc")
        .resources_dir_name("resources\\external\\")
        .window_setup(
            ggez::conf::WindowSetup::default()
                .title("Chess game")
                .samples(config.window.samples)
                .vsync(config.window.v_sync)
                // .icon("/icon/logoV1.png")// .icon(iconpath.as_str()), // .icon("/Python.ico"),
                .srgb(config.window.srgb),
        )
        .window_mode(config.window.into())
        .backend(ggez::conf::Backend::Dx12);

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
