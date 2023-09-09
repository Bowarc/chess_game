mod sound_id;
mod sound_source;

pub use sound_id::SoundId;

pub struct SoundBank {}

impl SoundBank {
    pub fn new() -> Self {
        Self {}
    }
}

impl super::Bank<SoundId, ggez::audio::Source> for SoundBank {
    fn update(&mut self) {
        todo!()
    }
    fn get(&mut self, _: SoundId, _: super::loader::Handle) -> &mut ggez::audio::Source {
        todo!()
    }
}
