mod connected;
mod connecting;
mod disconnected;
mod dummy;
mod game_end;
mod game_join;
mod game_leave;
mod game_start;
mod just_launched;
mod player_left;
mod playing;
mod waiting_for_opponent;

use connected::Connected;
use connecting::Connecting;
use disconnected::Disconnected;
use dummy::__Dummy;
use game_end::GameEnd;
use game_join::GameJoin;
use game_leave::GameLeave;
use game_start::GameStart;
use just_launched::JustLaunched;
use player_left::PlayerLeft;
use playing::Playing;
use waiting_for_opponent::WaitingForOpponent;

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
#[derive(enum_variant_name::VariantName)]
pub enum State {
    __Dummy,
    JustLaunched,
    Disconnected,
    Connecting,
    Connected,

    GameJoin,
    WaitingForOpponent,
    GameStart,
    Playing,
    GameEnd,
    PlayerLeft,
    GameLeave,
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

    pub fn from_shared_state(client: super::Client, game: shared::game::Game, my_id: shared::id::Id) -> Self {
        match game.state() {
            shared::game::State::PlayerDisconnected => PlayerLeft::new(client, my_id).into(),
            shared::game::State::Waiting => WaitingForOpponent::new(client, game, my_id).into(),
            shared::game::State::GameStart => GameStart::new(client, game, my_id).into(),
            shared::game::State::Playing { board: _ /* hmm */ } => Playing::new(client, game.id(), my_id).into(),
            shared::game::State::GameEnd { winner: _ /* hmm */} => GameEnd::new(client, game, my_id).into(),
        }
        // Might not be a bad idea to include those in the .new declaration
    }

    pub fn on_disconnect() -> Self {
        Disconnected::new().into()
    }
}
