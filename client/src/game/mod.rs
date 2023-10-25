mod state;

type Socket =
    crate::networking::Client<shared::message::ServerMessage, shared::message::ClientMessage>;

// The use of an enum is more fitting i think
#[derive(Default)]
pub enum Game {
    #[default]
    JustLaunched,
    Connected {
        socket: Socket,
    },
    Playing {
        socket: Socket,
    },
}

impl Game {
    pub fn update(&mut self) {}
}
