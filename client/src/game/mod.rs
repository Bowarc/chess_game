


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
        let this = std::mem::replace(&mut self.state, State::JustLaunched);

        match this{
            State::JustLaunched => {
                self.state = State::Disconnected {}  
            },
            State::Disconnected {  } => {
                self.state = State::Connecting { client: Client::new(shared::DEFAULT_ADDRESS) }
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
                    self.state = std::mem::replace(&mut self.state, State::Connecting { client })              
                }
            },
            State::Connected { mut client, active_games } => {
                client.update().unwrap();
                self.state = State::Connected { client, active_games }
                // self.state = State::Playing { client }
            },
            State::Playing { mut client } => {
                client.update().unwrap();
                self.state = State::Playing { client }
            },
        };

    }
}
