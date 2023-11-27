pub struct JustLaunched {}

impl super::StateMachine for JustLaunched {
    fn update(self, _ggctx: &mut ggez::Context, _delta_time: f64) -> super::State {
        super::Disconnected::new().into()
    }

    fn draw(self, _: &mut crate::render::RenderRequest) -> super::State {
        self.into()
    }
}
