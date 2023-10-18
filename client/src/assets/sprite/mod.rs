mod sprite_id;
pub use sprite_id::SpriteId;

pub struct SpriteBank {
    sprites: std::collections::HashMap<SpriteId, ggez::graphics::InstanceArray>,
}

impl SpriteBank {
    pub fn new(loader_handle: &mut super::loader::Handle) -> Self {
        loader_handle.request(super::loader::Request::Sprite(SpriteId::default()));
        Self {
            sprites: std::collections::HashMap::default(),
        }
    }
}

impl super::Bank<SpriteId, ggez::graphics::InstanceArray> for SpriteBank {
    fn update(&mut self, ctx: &mut ggez::Context, loader_handle: &mut super::loader::Handle) {
        if let Some(response) = loader_handle.retrieve_data(super::loader::TargetId::Sprite) {
            let id = response.request.sprite_id();
            debug!("Sprite bank received data for {id:?}",);

            let img = match ggez::graphics::Image::from_bytes(ctx, &response.bytes) {
                Ok(img) => img,
                Err(e) => {
                    error!(
                        "Sprite bank could not parse received data for sprite {id:?}, reason: {e}",
                    );
                    return;
                }
            };

            self.sprites
                .insert(id, ggez::graphics::InstanceArray::new(ctx, img));
        }
    }
    fn try_get_mut(
        &mut self,
        id: &SpriteId,
        loader_handle: &mut super::loader::Handle,
    ) -> Option<&mut ggez::graphics::InstanceArray> {
        if self.sprites.contains_key(id) {
            return self.sprites.get_mut(id);
        }

        loader_handle.request(super::loader::Request::Sprite(*id));

        if self.sprites.contains_key(&SpriteId::default()) {
            return self.sprites.get_mut(&SpriteId::default());
        }

        None
    }
}
