pub struct JustLaunched {}

impl super::StateMachine for JustLaunched {
    fn update(mut self, _delta_time: f64) -> super::State {
        super::Disconnected::new().into()
    }

    fn draw(mut self, _: &mut crate::render::RenderRequest) -> super::State {
        self.into()
    }
}
