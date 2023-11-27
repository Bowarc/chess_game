mod connected;
mod connecting;
mod disconnected;
mod dummy;
mod just_launched;
mod playing;

use connected::Connected;
use connecting::Connecting;
use disconnected::Disconnected;
use dummy::__Dummy;
use just_launched::JustLaunched;
use playing::Playing;

#[enum_dispatch::enum_dispatch]
pub trait StateMachine: Sized {
    fn update(self, ggctx: &mut ggez::Context, delta_time: f64) -> State;
    fn draw(self, _: &mut crate::render::RenderRequest) -> State;

    fn try_get_client_mut(&mut self) -> Option<&mut super::Client> {
        None
    }
    fn try_get_ui_mgr_mut(&mut self) -> Option<&mut crate::ui::UiManager> {
        None
    }
}

#[enum_dispatch::enum_dispatch(StateMachine)]
pub enum State {
    __Dummy,
    JustLaunched,
    Disconnected,
    Connecting,
    Connected,
    Playing,
}

impl Default for State {
    fn default() -> Self {
        JustLaunched {}.into()
    }
}

impl State {
    pub fn dummy() -> Self {
        __Dummy.into()
    }
    pub fn name(&self) -> &'static str {
        match self {
            State::__Dummy(_) => "__Dummy",
            State::JustLaunched(_) => "JustLaunched",
            State::Disconnected(_) => "Disconnected",
            State::Connecting(_) => "Connecting",
            State::Connected(_) => "Connected",
            State::Playing(_) => "Playing",
        }
    }
}
