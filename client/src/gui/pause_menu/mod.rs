use crate::assets::file;
use crate::config;
use crate::gui;
use ggegui::egui;
use ron::de;

mod config_view;
mod credits_view;

const WINDOW_SIZE: (f32, f32) = (300., 130.);

const ARCHITECTURE_FILE: file::ConsPath =
    file::ConsPath::new(file::FileSystem::Internal, "config/menuArchitecture.ron");

#[derive(Debug, Clone)]
pub struct OverTimeValues {
    graphic_context_size_slider_index: f32,
    supported_resolutions: Vec<ggez::winit::dpi::PhysicalSize<u32>>,
    selected_fullscreen_type: ggez::conf::FullscreenType,
}
impl OverTimeValues {
    fn new(ctx: &mut ggez::Context, cfg: &config::Config) -> Self {
        Self {
            graphic_context_size_slider_index: 0.,
            supported_resolutions: {
                let mut r = Vec::new();

                let supported_res: Vec<ggez::winit::dpi::PhysicalSize<u32>> =
                    ctx.gfx.supported_resolutions().collect();

                for res in supported_res {
                    if !r.contains(&res) {
                        r.push(res)
                    }
                }
                r
            },
            selected_fullscreen_type: cfg.window.fullscreen_type,
        }
    }
}

#[derive(serde::Deserialize, PartialEq, Eq, Clone, Copy, Debug)]
pub enum UiState {
    None,
    TheActualGame,     // Launches the game with settings
    MainMenu,          // starting menu
    NewOrContinueMenu, // choose to create a new game or continue a save (aslo the play button menu)
    // SelectGameMode, // eeeeh we're far from something like that for now
    // ContinueGame,   // useless to have an ui for that as it only launches the game
    CreateNewGameMenu, // settings menu to create a new game: (Chat difficulty, modifiers?, character?)
    GameSettingsMenu,  // menu to modify settings for the windows or the game in a global way
    Credits,           // See credits menu
    PlayerStats,       // See player stat menu
}

pub struct PauseMenu {
    otv: OverTimeValues,
    architecture: gui::architecture::Architecture<UiState>,

    animation: gui::animation::WindowAnimation,
    visibility: gui::animation::WindowVisibility,
    scale: f64,
    show: bool,
    anchor: (f32, f32),
}

impl PauseMenu {
    pub fn new(ctx: &mut ggez::Context, cfg: &config::Config) -> ggez::GameResult<Self> {
        // let Ok(architecture_bytes) = file::try_bytes(ARCHITECTURE_FILE.into()) else{
        //     error!("GUI got an error while loading pause menu architecture file: {e:?}");
        //     return Err(ggez::GameError::FilesystemError(format!("{e:?}")));
        // };

        // let Ok(architecture) = de::from_bytes::<gui::architecture::Architecture>(&architecture_bytes) else{
        //     error!("GUI got an error while parsing pause menu architecture bytes: {e:?}");
        //     return Err(ggez::GameError::ResourceLoadError(format!("{e:?}")));
        // };

        let architecture_bytes = match file::try_bytes(ARCHITECTURE_FILE.into()) {
            Ok(bytes) => bytes,
            Err(e) => {
                error!("Got an error while loading pause menu architecture file: {e:?}");
                return Err(ggez::GameError::FilesystemError(format!("{e:?}")));
            }
        };

        let architecture =
            match de::from_bytes::<gui::architecture::Architecture<UiState>>(&architecture_bytes) {
                Ok(architecture) => architecture,
                Err(e) => {
                    error!("got an error while parsing pause menu architecture bytes: {e:?}");
                    return Err(ggez::GameError::ResourceLoadError(format!("{e:?}")));
                }
            };

        debug!("Loaded menu architecture: {architecture:?}");

        Ok(PauseMenu {
            animation: gui::animation::WindowAnimation {
                opening_time: 300.,
                closing_time: 80.,
                state: gui::animation::AnimationState::Close,
                start_time: std::time::Instant::now(),
            },
            visibility: gui::animation::WindowVisibility {
                last_switch: std::time::Instant::now(),
                switch_delay: 200,
            },
            scale: 4.,
            show: false,
            architecture,
            otv: OverTimeValues::new(ctx, cfg),
            anchor: (0., 0.),
        })
    }

    pub fn is_active(&self) -> bool {
        self.show
    }

    fn update_animation(&mut self, ggez_ctx: &mut ggez::Context, cfg: &crate::config::Config) {
        use crate::action;
        use crate::input;

        // if user pressed the key AND it has been longer than the animation time since the last switch
        if input::pressed(
            ggez_ctx,
            *cfg.user
                .input_binds
                .get(&action::Action::UiPauseMenuOpen)
                .expect("GUI could not get the keybind for Action::UiPauseMenuOpen"),
        ) && self.visibility.last_switch.elapsed().as_millis()
            > self.visibility.switch_delay as u128
        {
            match self.animation.state {
                gui::animation::AnimationState::Open => {
                    self.animation.start_time = std::time::Instant::now();
                    self.visibility.last_switch = std::time::Instant::now();
                    self.animation.state = gui::animation::AnimationState::Closing
                }
                gui::animation::AnimationState::Close => {
                    self.architecture.actual = UiState::MainMenu;

                    self.animation.start_time = std::time::Instant::now();
                    self.visibility.last_switch = std::time::Instant::now();
                    self.animation.state = gui::animation::AnimationState::Opening
                }
                _ => {}
            }
        }

        // let window_size = (700., 300.);
        let window_size = WINDOW_SIZE;

        // Total time of the current animation
        let anim_time = match self.animation.state {
            gui::animation::AnimationState::Opening => self.animation.opening_time,
            gui::animation::AnimationState::Closing => self.animation.closing_time,
            _ => 0.,
        };

        let anchor = if self.animation.start_time.elapsed().as_millis() < anim_time as u128 {
            // fraction of the current animation
            let fraction: f32 =
                ((anim_time as u128 - self.animation.start_time.elapsed().as_millis()) as f32)
                    / anim_time;

            match self.animation.state {
                gui::animation::AnimationState::Opening => (0., fraction * -window_size.1),
                gui::animation::AnimationState::Closing => (0., (fraction - 1.) * window_size.1),

                _ => (0., 0.),
            }
        } else {
            self.animation.state = match self.animation.state {
                gui::animation::AnimationState::Opening => gui::animation::AnimationState::Open,
                gui::animation::AnimationState::Closing => gui::animation::AnimationState::Close,
                _ => self.animation.state,
            };
            (0., 0.)
        };

        self.anchor = anchor;
        // debug!("Anchor: {:?}", self.anchor);

        self.show = self.animation.state != gui::animation::AnimationState::Close
    }

    pub fn update(
        &mut self,
        ggez_ctx: &mut ggez::Context,
        egui_ctx: &egui::Context,
        cfg: &mut crate::config::Config,
    ) {
        self.update_animation(ggez_ctx, cfg);

        let window = egui::Window::new("Pause menu")
            .resizable(false)
            .title_bar(false)
            .collapsible(false)
            .anchor(egui::Align2::LEFT_TOP, self.anchor)
            // .default_size((window_size.0, window_size.1))
            .fixed_size(WINDOW_SIZE);
        window
            .open(&mut self.show)
            // .vscroll(true)
            // .hscroll(true)
            .show(egui_ctx, |ui| {
                match self.architecture.actual {
                    UiState::None => {
                        error!(
                            "Current architecture bit is UiState::None, reseting to the first one"
                        );
                        self.architecture.reset()
                    }
                    UiState::TheActualGame => {
                        // Nothing to draw that is not a children (therefore already handled by the architecture.draw_childs call)
                    }
                    UiState::MainMenu => {
                        // Nothing to draw that is not a children (therefore already handled by the architecture.draw_childs call)
                    }
                    UiState::NewOrContinueMenu => {
                        // Nothing to draw that is not a children (therefore already handled by the architecture.draw_childs call)
                    }
                    UiState::CreateNewGameMenu => {
                        // Nothing to draw that is not a children (therefore already handled by the architecture.draw_childs call)
                    }
                    UiState::GameSettingsMenu => {
                        config_view::draw_config_menu(ggez_ctx, ui, cfg, &mut self.otv)
                    }

                    UiState::Credits => credits_view::draw_credits(ui),
                    UiState::PlayerStats => {
                        // Nothing to draw that is not a children (therefore already handled by the architecture.draw_childs call)
                    }
                }

                self.architecture.draw_childs(ui);

                if ui.button("Quit game").clicked() {
                    ggez_ctx.request_quit()
                }
            });
    }
}
