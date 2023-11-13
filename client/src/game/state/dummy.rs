pub struct __Dummy;

impl super::StateMachine for __Dummy{
    fn update(mut self,delta_time:f64) -> super::State {
        unreachable!("You're not supposed to use that variant")
    }

    fn draw(mut self,_: &mut crate::render::RenderRequest) -> super::State {
        unreachable!("You're not supposed to use that variant")
    }
}