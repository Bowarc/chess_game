// config file are in external fs

const USER_CONFIG_FILE: shared::file::ConsPath = shared::file::ConsPath::new(
    shared::file::FileSystem::External,
    "config\\globalConfig.ron",
);

#[derive(derivative::Derivative, serde::Deserialize, Debug, Clone)]
#[serde(default)]
#[derivative(Default)]
pub struct Config {
    #[derivative(Default(value = "UserConfig::default()"))]
    pub user: UserConfig,
    #[derivative(Default(value = "WindowConfig::default()"))]
    pub window: WindowConfig,
    #[derivative(Default(value = "RenderConfig::default()"))]
    pub render: RenderConfig,
    #[derivative(Default(value = "AudioConfig::default()"))]
    pub audio: AudioConfig,
    #[derivative(Default(value = "GUIConfig::default()"))]
    pub gui: GUIConfig,
    #[derivative(Default(value = "false"))]
    pub gamepad: bool,
    #[derivative(Default(value = "OptimisationConfig::default()"))]
    pub optimisation: OptimisationConfig, // threading: ThreadingConfig?
}

#[derive(derivative::Derivative, serde::Deserialize, Debug, Clone)]
#[serde(default)]
#[derivative(Default)]
pub struct UserConfig {
    #[derivative(Default(value = "[
            (crate::action::Action::UiPauseMenuOpen, crate::input::Input::KeyboardEscape)
            ].iter().cloned().collect()"))]
    pub input_binds: std::collections::HashMap<crate::action::Action, crate::input::Input>,
}

#[derive(derivative::Derivative, serde::Deserialize, Debug, Clone, Copy)]
#[serde(default)]
#[derivative(Default)]
pub struct WindowConfig {
    #[derivative(Default(value = "false"))]
    pub v_sync: bool,
    #[derivative(Default(value = "(1920, 1080)"))]
    pub size: (i32, i32),
    #[derivative(Default(value = "ggez::conf::FullscreenType::Desktop"))]
    pub fullscreen_type: ggez::conf::FullscreenType,
    #[derivative(Default(value = "ggez::conf::NumSamples::One"))]
    pub samples: ggez::conf::NumSamples,
    #[derivative(Default(value = "true"))]
    pub srgb: bool,
    #[derivative(Default(value = "false"))]
    pub resizable: bool,
}
#[derive(derivative::Derivative, serde::Deserialize, Debug, Clone, Copy)]
#[serde(default)]
#[derivative(Default)]
pub struct RenderConfig {}

#[derive(derivative::Derivative, serde::Deserialize, Debug, Clone, Copy)]
#[serde(default)]
#[derivative(Default)]
pub struct AudioConfig {
    #[derivative(Default(value = "false"))]
    pub enabled: bool,
    #[derivative(Default(value = "0.5"))]
    pub global_vol: f64,
    #[derivative(Default(value = "0.5"))]
    pub music_vol: f64,
    #[derivative(Default(value = "0.5"))]
    pub gameplay_vol: f64,
}

#[derive(derivative::Derivative, serde::Deserialize, Debug, Clone, Copy)]
#[serde(default)]
#[derivative(Default)]
pub struct GUIConfig {
    #[derivative(Default(value = "3."))]
    pub scale: f64,
}

#[derive(derivative::Derivative, serde::Deserialize, Debug, Clone, Copy)]
#[serde(default)]
#[derivative(Default)]
pub struct OptimisationConfig {}

pub fn load() -> Config {
    // check config/default/ for default config, then check for a user one
    // Not done like this for now

    let config = match shared::file::try_bytes(USER_CONFIG_FILE.into()) {
        Ok(bytes) => match ron::de::from_bytes(&bytes) {
            Ok(s) => s,
            Err(e) => {
                error!("An error occured while parsing the config file data: {}\n\tFalling back to default",e);
                Default::default()
            }
        },
        Err(_e) => Default::default(),
    };

    config
}

#[derive(Default, Debug, PartialEq, Eq, serde::Deserialize, Clone, Copy)]
pub enum DashDirectionMethod {
    #[default]
    Movement,
    Cursor,
}

impl From<WindowConfig> for ggez::conf::WindowMode {
    fn from(window_config: WindowConfig) -> ggez::conf::WindowMode {
        ggez::conf::WindowMode {
            width: window_config.size.0 as f32,
            height: window_config.size.1 as f32,
            maximized: false,
            fullscreen_type: window_config.fullscreen_type,
            borderless: false,
            min_width: 1.0,
            max_width: 0.0,
            min_height: 1.0,
            max_height: 0.0,
            resizable: window_config.resizable,
            visible: true,
            transparent: false,
            resize_on_scale_factor_change: false,
            logical_size: None,
        }
    }
}
