mod sprite_id;
pub use sprite_id::SpriteId;

pub struct SpriteBank {}

impl SpriteBank {
    pub fn new() -> Self {
        Self {}
    }
}

impl super::Bank<SpriteId, ggez::graphics::InstanceArray> for SpriteBank {
    fn update(&mut self) {
        todo!()
    }
    fn get(&mut self, _: SpriteId, _: super::loader::Handle) -> &mut ggez::graphics::InstanceArray {
        todo!()
    }
}
