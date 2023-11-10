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
        match &mut self.state {
            State::Connecting { client, .. }
            | State::Connected { client, .. }
            | State::Playing { client, .. } => Some(client),
            _ => None,
        }
    }
    pub fn try_get_ui_mgr_mut(&mut self) -> Option<&mut crate::ui::UiManager> {
        match &mut self.state {
            State::Disconnected { ui }
            | State::Connecting { ui, .. }
            | State::Connected { ui, .. }
            | State::Playing { ui, .. } => Some(ui),
            _ => None,
        }
    }
    pub fn update(&mut self) {
        // take ownership of the old status, installing the dummy Idle
        let state_disc = std::mem::discriminant(&self.state);
        match &self.state {
            State::JustLaunched => self.update_just_launched(),
            State::Disconnected { .. } => {
                self.update_disconnected();
            }
            State::Connecting { .. } => {
                self.update_connecting();
            }
            State::Connected { .. } => {
                self.update_connected();
            }
            State::Playing { .. } => self.update_playing(),
            State::__Dummy => unreachable!("Dummy state cannot be used as value"),
        };
        if let State::__Dummy = self.state {
            panic!(
                "You forgot to switch back the state, {:?}",
                stringify!(self.state)
            );
        }
    }

    fn update_just_launched(&mut self) {
        self.state = State::new_disconnected();
    }
    fn update_disconnected(&mut self) {
        let State::Disconnected {
            ui 
        } = std::mem::replace(&mut self.state, State::__Dummy) else {
            panic!()
        };
        if let Ok(client) = Client::new(shared::DEFAULT_ADDRESS) {
            self.state = State::new_connecting(client);
        } else {
            warn!("Could not connect to the sever..");
            self.state = State::Disconnected { ui }
        }
    }
    fn update_connecting(&mut self) {
        let State::Connecting {
            ui,
            mut client
        } = std::mem::replace(&mut self.state, State::__Dummy) else{
            panic!()
        };
        client.update().unwrap();
        if client.is_connected() {
            debug!("Client is now connected, switching State to connected");
            self.state = State::new_connected(client);
        } else {
            warn!("Still trying to connect");
            self.state = State::Connecting { ui, client };
            // let dummy_state = std::mem::replace(&mut self.state, State::Connecting { client });
        }
    }
    fn update_connected(&mut self) {
        let State::Connected {
            mut ui, 
            mut client,
            mut active_games
        } = std::mem::replace(&mut self.state, State::__Dummy) else{
            panic!()
        };

        if let Err(e) = client.update() {
            self.state = State::new_disconnected();
            return;
        }
        active_games.update(&mut client);
        if active_games.changed() {
            let text_id = ui.add_element(crate::ui::element::Element::new_text(
                "Test text 1",
                crate::ui::Anchor::TopCenter,
                20.,
                crate::ui::Style::default(),
                vec![crate::ui::element::TextBit::new_text("Salut", None)],
            ));
            debug!("Added new text with id: {text_id}");
        }
        self.state = State::Connected {
            ui,
            client,
            active_games,
        }

        // self.state = State::Playing { client }
    }
    fn update_playing(&mut self) {
        let State::Playing {
            ui,
            mut client,
            current_game,
            current_board
        } = std::mem::replace(&mut self.state, State::__Dummy) else{
            panic!()
        };
        if let Err(e) = client.update() {
            error!("Got an error while updating the connection with the server: {e}");
            self.state = State::new_disconnected();
        } else {
            self.state = State::Playing {
                ui,
                client,
                current_game,
                current_board,
            }
        }
    }
}
