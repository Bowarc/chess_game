pub struct GameJoin {
    client: crate::game::Client,
    game: shared::game::Game,
}

impl GameJoin {
    pub fn new(client: crate::game::Client, game: shared::game::Game) -> Self {
        debug!("Creating GameJoin State");
        Self { client, game }
    }
}

impl super::StateMachine for GameJoin {
    fn update(self, ggctx: &mut ggez::Context, delta_time: f64) -> super::State {
        super::State::from_shared_state(self.client, self.game)
    }

    fn draw(self, _: &mut crate::render::RenderRequest) -> super::State {
        // Nothing to draw, should be quick
        self.into()
    }
}
