mod state;
use state::State;

type Client =
    crate::networking::Client<shared::message::ServerMessage, shared::message::ClientMessage>;

pub struct Game {
    state: state::State,
}
impl Game {
    pub fn new() -> Self {
        Self {
            state: State::default(),
        }
    }
    pub fn try_get_client_mut(&mut self) -> Option<&mut Client> {
        state::StateMachine::try_get_client_mut(&mut self.state)
    }
    pub fn try_get_ui_mgr_mut(&mut self) -> Option<&mut crate::ui::UiManager> {
        state::StateMachine::try_get_ui_mgr_mut(&mut self.state)
    }
    pub fn update(&mut self, delta_time: f64) {
        self.state = state::StateMachine::update(
            std::mem::replace(&mut self.state, State::dummy()),
            delta_time,
        );
        self.verify_state()
    }
    pub fn draw(&mut self, render_request: &mut crate::render::RenderRequest) {
        self.state = state::StateMachine::draw(
            std::mem::replace(&mut self.state, State::dummy()),
            render_request,
        );
        self.verify_state()
    }
    pub fn verify_state(&self) {
        if let State::__Dummy(_) = self.state {
            panic!("Dummy state detected, you might have forgot to switch it back",);
        }
    }
}
