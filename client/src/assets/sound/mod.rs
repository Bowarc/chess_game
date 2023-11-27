mod sound_id;
mod sound_source;

pub use sound_id::SoundId;

pub struct SoundBank {}

impl SoundBank {
    pub fn new(_loader_handle: &mut super::loader::Handle) -> Self {
        Self {}
    }
}

impl super::Bank<SoundId, ggez::audio::Source> for SoundBank {
    fn update(&mut self, _ctx: &mut ggez::Context, _loader_handle: &mut super::loader::Handle) {
        // nothing to do for now
    }

    fn try_get_mut(
        &mut self,
        _: &SoundId,
        _: &mut super::loader::Handle,
    ) -> std::option::Option<&mut ggez::audio::Source> {
        todo!()
    }
}
