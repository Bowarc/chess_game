pub struct Player{
    // id: shared::id::Id,
    client: crate::networking::Client<shared::message::ClientMessage, shared::message::ServerMessage>,
    
}


impl Player{
    pub fn new(client: crate::networking::Client<shared::message::ClientMessage, shared::message::ServerMessage>) -> Self{
        Self{
            // id: shared::id::Id::new(),
            client,
        }
    }

    pub fn id(&self) -> shared::id::Id{
        self.client.id()
    }

    pub fn update(&mut self){
        self.client.update().unwrap();
    }
}