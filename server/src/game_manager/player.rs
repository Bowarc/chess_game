pub struct Player {
    // id: shared::id::Id,
    client:
        crate::networking::Client<shared::message::ClientMessage, shared::message::ServerMessage>,
    name: String,
}

impl Player {
    pub fn new(
        client: crate::networking::Client<
            shared::message::ClientMessage,
            shared::message::ServerMessage,
        >,

    ) -> Self {
        Self {
            name: format!("Player{}", client.id()),
            client,
        }
    }

    pub fn id(&self) -> shared::id::Id {
        self.client.id()
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn is_connected(&self) -> bool {
        self.client.is_connected() && self.client.is_running()
    }

    pub fn try_recv(
        &mut self,
    ) -> Result<shared::message::ClientMessage, std::sync::mpsc::TryRecvError> {
        self.client.try_recv()
    }

    pub fn send(
        &mut self,
        msg: shared::message::ServerMessage,
    ) -> Result<(), std::boxed::Box<std::sync::mpsc::SendError<shared::message::ServerMessage>>>{
        Ok(self.client.send(msg)?)
    }
}

impl From<&Player> for shared::game::Player {
    fn from(server_player: &Player) -> Self {
        shared::game::Player::new(server_player.id(), server_player.name())
    }
}
