pub struct GameJoin {
    client: crate::game::Client,
    game: shared::game::Game,
    my_id: shared::id::Id,
}

impl GameJoin {
    pub fn new(
        client: crate::game::Client,
        game: shared::game::Game,
        my_id: shared::id::Id,
    ) -> Self {
        debug!("Creating GameJoin State");
        Self {
            client,
            game,
            my_id,
        }
    }
}

impl super::StateMachine for GameJoin {
    fn update(self, ggctx: &mut ggez::Context, delta_time: f64) -> super::State {
        super::State::from_shared_state(self.client, self.game, self.my_id)
    }

    fn draw(self, _: &mut crate::render::RenderRequest) -> super::State {
        // Nothing to draw, should be quick
        self.into()
    }
}
