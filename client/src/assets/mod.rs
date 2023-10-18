pub use shared::file;
pub mod font;
pub mod loader;
pub mod sound;
pub mod sprite;

pub trait Bank<In, Out> {
    fn update(&mut self, _: &mut ggez::Context, _: &mut loader::Handle);
    fn try_get_mut(&mut self, _: &In, _: &mut loader::Handle) -> Option<&mut Out>;
    fn get_mut(&mut self, input: &In, loader_handle: &mut loader::Handle) -> &mut Out {
        self.try_get_mut(input, loader_handle).unwrap()
    }
}

pub struct AssetManager {
    pub sprite_bank: sprite::SpriteBank,
    sound_bank: sound::SoundBank,
    pub loader_handle: loader::Handle,
}

impl AssetManager {
    pub fn new() -> Self {
        let mut loader_handle = loader::init();

        let sprite_bank = sprite::SpriteBank::new(&mut loader_handle);
        let sound_bank = sound::SoundBank::new(&mut loader_handle);

        debug!("Succesfully created asset banks and loader thread");

        Self {
            sprite_bank,
            sound_bank,
            loader_handle,
        }
    }
    pub fn get_loader_mut(&mut self) -> &mut loader::Handle {
        &mut self.loader_handle
    }
    pub fn get_loader(&self) -> &loader::Handle {
        &self.loader_handle
    }

    pub fn update(&mut self, ctx: &mut ggez::Context) {
        self.loader_handle.update();
        self.sprite_bank.update(ctx, &mut self.loader_handle);
        self.sound_bank.update(ctx, &mut self.loader_handle);
    }
}

// Bank getters
impl AssetManager {
    pub fn get_sprite_bank(&self) -> &impl Bank<sprite::SpriteId, ggez::graphics::InstanceArray> {
        &self.sprite_bank
    }
    pub fn get_sprite_bank_mut(
        &mut self,
    ) -> &mut impl Bank<sprite::SpriteId, ggez::graphics::InstanceArray> {
        &mut self.sprite_bank
    }
    pub fn get_sound_bank(&self) -> &impl Bank<sound::SoundId, ggez::audio::Source> {
        &self.sound_bank
    }
    pub fn get_sound_bank_mut(&mut self) -> &mut impl Bank<sound::SoundId, ggez::audio::Source> {
        &mut self.sound_bank
    }
}
