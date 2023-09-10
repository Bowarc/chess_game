pub use shared::file;
pub mod font;
pub mod loader;
pub mod sound;
pub mod sprite;

pub trait Bank<In, Out> {
    fn update(&mut self);
    fn get(&mut self, _: In, _: loader::Handle) -> &mut Out;
}

pub struct AssetManager {
    sprite_bank: sprite::SpriteBank,
    sound_bank: sound::SoundBank,
    loader_handle: loader::Handle,
}

impl AssetManager {
    pub fn new() -> Self {
        let sprite_bank = sprite::SpriteBank::new();
        let sound_bank = sound::SoundBank::new();

        let loader_handle = loader::init();

        debug!("Succesfully created asset banks and loader thread");

        Self {
            sprite_bank,
            sound_bank,
            loader_handle,
        }
    }
    pub fn loader(&self) -> &loader::Handle {
        &self.loader_handle
    }
}
