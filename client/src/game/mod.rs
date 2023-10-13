mod state;

type Socket =
    crate::networking::Client<shared::message::ServerMessage, shared::message::ClientMessage>;

// The use of an enum is more fitting i think
pub enum Game {
    JustLaunched,
    Connected { socket: Socket },
    Playing { socket: Socket },
}
