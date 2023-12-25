pub struct GameLeave {
    client: crate::game::Client,
    my_id: shared::id::Id,
}

impl GameLeave {
    pub fn new(client: crate::game::Client,my_id: shared::id::Id,) -> Self {
        debug!("Creating GameLeave State");
        Self { client, my_id }
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
