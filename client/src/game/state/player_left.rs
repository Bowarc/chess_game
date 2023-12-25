pub struct PlayerLeft {
    client: crate::game::Client,
    my_id: shared::id::Id
}

impl PlayerLeft {
    pub fn new(client: crate::game::Client, my_id: shared::id::Id) -> Self {
        debug!("Creating PlayerLeft State");
        Self { client, my_id }
    }
}

impl super::StateMachine for PlayerLeft {
    fn update(mut self, ggctx: &mut ggez::Context, delta_time: f64) -> super::State {
        if !self.client.is_connected() {
            warn!("Client has been disconnected");
            return super::State::on_disconnect();
        }
        if let Err(e) = self.client.update() {
            error!("Got an error while updating the connection with the server: {e}");
            return super::State::on_disconnect();
        }
        self.into()
    }

    fn draw(self, _: &mut crate::render::RenderRequest) -> super::State {
        self.into()
    }
}
