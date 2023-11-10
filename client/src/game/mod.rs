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
            State::Connecting { client } |
            State::Connected { client, .. } |
            State::Playing { client }=> {
                Some(client)
            },
            _ => None
        } 
    }
    pub fn update(&mut self) {
        // take ownership of the old status, installing the dummy Idle
        let state = std::mem::replace(&mut self.state, State::__Dummy);
        let state_disc = std::mem::discriminant(&state);
        match state{
            State::JustLaunched => {
                self.state = State::Disconnected {}  
            },
            State::Disconnected {  } => {
                if let Ok(client) = Client::new(shared::DEFAULT_ADDRESS){
                    self.state = State::Connecting { client  }
                }else{
                    warn!("Could not connect to the sever..");
                    self.state = State::Disconnected {  }
                }
            },
            State::Connecting { mut client } => {
                client.update().unwrap();
                if client.is_connected(){
                    error!("Client is now connected, switching State to connected");
                    self.state = State::Connected {
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
                    self.state = State::Connecting { client };
                    // let dummy_state = std::mem::replace(&mut self.state, State::Connecting { client });              
                }
            },
            State::Connected { mut client, active_games } => {
                if let Err(e) = client.update(){
                    self.state = State::Disconnected {  }

                }else{

                    self.state = State::Connected { client, active_games }
                }
                // self.state = State::Playing { client }
            },
            State::Playing { mut client } => {
                client.update().unwrap();
                self.state = State::Playing { client }
            },
            State::__Dummy => unreachable!("Dummy state cannot be used as value")
        };
        if let State::__Dummy = self.state{
            panic!("You forgot to switch back the state, {state_disc:?}");
        }

    }
}
