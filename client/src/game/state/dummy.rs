pub struct __Dummy;

impl super::StateMachine for __Dummy {
    fn update(self, _ggctx: &mut ggez::Context, _delta_time: f64) -> super::State {
        unreachable!("You're not supposed to use that variant")
    }

    fn draw(self, _: &mut crate::render::RenderRequest) -> super::State {
        unreachable!("You're not supposed to use that variant")
    }
}
