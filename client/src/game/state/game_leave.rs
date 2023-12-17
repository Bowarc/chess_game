pub struct GameLeave {
    client: crate::game::Client,
}

impl GameLeave {
    pub fn new(client: crate::game::Client) -> Self {
        debug!("Creating GameLeave State");
        Self { client }
    }
}

impl super::StateMachine for GameLeave {
    fn update(self, ggctx: &mut ggez::Context, delta_time: f64) -> super::State {
        todo!()
    }

    fn draw(self, _: &mut crate::render::RenderRequest) -> super::State {
        todo!()
    }
}
