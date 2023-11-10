mod state;
use state::State;

type Client =
    crate::networking::Client<shared::message::ServerMessage, shared::message::ClientMessage>;


pub struct Game{
    state: state::State
}
impl Game {
    pub fn new() -> Self {
        Self{
            state: State::default(),
        }
    }
    pub fn try_get_client_mut(&mut self) -> Option<&mut Client>{
        match &mut self.state{
            State::Connecting { client, .. } |
            State::Connected { client, .. } |
            State::Playing { client, .. }=> {
                Some(client)
            },
            _ => None
        } 
    }
    pub fn try_get_ui_mgr(&self) -> Option<&crate::ui::UiManager>{

        None
    }
    pub fn update(&mut self) {
        // take ownership of the old status, installing the dummy Idle
        let state = std::mem::replace(&mut self.state, State::__Dummy);
        let state_disc = std::mem::discriminant(&state);
        match state{
            State::JustLaunched => {
                let ui = State::default_disconnected_ui();
                self.state = State::Disconnected {ui}  
            },
            State::Disconnected { ui } => {
                self.update_disconnected(ui);
            },
            State::Connecting { ui, client } => {
                self.update_connecting(ui, client);
            },
            State::Connected { ui, client, active_games } => {
                self.update_connected(ui, client, active_games);
            },
            State::Playing { ui, client, current_game, current_board } => {
                self.update_playing(ui, client, current_game, current_board)
            },
            State::__Dummy => unreachable!("Dummy state cannot be used as value")
        };
        if let State::__Dummy = self.state{
            panic!("You forgot to switch back the state, {state_disc:?}");
        }
    }

    fn update_just_lanched(&mut self){
        let ui = State::default_disconnected_ui();
        self.state = State::Disconnected {ui};
    }
    fn update_disconnected(&mut self, ui: crate::ui::UiManager){
        if let Ok(client) = Client::new(shared::DEFAULT_ADDRESS){
            self.state = State::Connecting { client, ui: State::default_connecting_ui() }
        }else{
            warn!("Could not connect to the sever..");
            self.state = State::Disconnected { ui }
        }
    }
    fn update_connecting(&mut self, ui: crate::ui::UiManager, mut client: Client){
        client.update().unwrap();
        if client.is_connected(){
            error!("Client is now connected, switching State to connected");
            self.state = State::Connected {
                ui: State::default_connected_ui(),
                client,
                active_games: super::networking::Future::new(
                    shared::message::ClientMessage::RequestGames,
                    |msg|matches!(msg, shared::message::ServerMessage::Games(_)),
                    |msg|{
                        if let shared::message::ServerMessage::Games(g) = msg{
                            g
                        }else{
                            panic!("?? Something went wrong in the validator ");
                        }
                    }
                ) 
            }
        }else{
            warn!("Still trying to connect");
            self.state = State::Connecting { ui, client };
            // let dummy_state = std::mem::replace(&mut self.state, State::Connecting { client });              
        }
    }
    fn update_connected(&mut self, ui: crate::ui::UiManager, mut client: Client, active_games: crate::networking::Future<Vec<shared::game::Game>>){
        if let Err(e) = client.update(){
            self.state = State::Disconnected { ui: State::default_disconnected_ui() }
        }else{
            self.state = State::Connected { ui, client, active_games }
        }
        // self.state = State::Playing { client }
    }
    fn update_playing(&mut self, ui: crate::ui::UiManager, mut client: Client, current_game: crate::networking::Future<shared::game::Game>, current_board: crate::networking::Future<shared::chess::Board>){
        if let Err(e) = client.update(){
            error!("Got an error while updating the connection with the server: {e}");
            self.state = State::Disconnected { ui: State::default_disconnected_ui() } 
        }else{
            self.state = State::Playing { ui, client, current_game, current_board }
        }
    }
}
