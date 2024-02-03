#[derive(Debug, Clone, Copy)]
pub struct Background {
    color: crate::render::Color,
    img: Option<crate::assets::sprite::SpriteId>,
}

impl Background {
    pub fn new(color: crate::render::Color, img: Option<crate::assets::sprite::SpriteId>) -> Self {
        Self { color, img }
    }

    pub fn get_color_mut(&mut self) -> &mut crate::render::Color {
        &mut self.color
    }

    pub fn get_color(&self) -> &crate::render::Color {
        &self.color
    }

    pub fn get_img_mut(&mut self) -> Option<&mut crate::assets::sprite::SpriteId> {
        self.img.as_mut()
    }

    pub fn get_img(&self) -> Option<&crate::assets::sprite::SpriteId> {
        self.img.as_ref()
    }

    pub fn draw(
        &self,
        mesh: &mut ggez::graphics::MeshBuilder,
        render_request: &mut crate::render::RenderRequest,
        element_rect: shared::maths::Rect,
    ) -> ggez::GameResult {
        if let Some(sprite_id) = self.get_img() {
            render_request.add(
                *sprite_id,
                crate::render::DrawParam::default()
                    .pos(element_rect.center())
                    .size(element_rect.size())
                    .color(*self.get_color()),
                crate::render::Layer::UiBackground,
            )
        } else {
            mesh.rectangle(
                ggez::graphics::DrawMode::fill(),
                element_rect.into(),
                (*self.get_color()).into(),
            )?;
        }

        Ok(())
    }
}

impl Default for Background {
    fn default() -> Self {
        Background {
            color: crate::render::Color::from_rgb(24, 24, 24),
            img: None,
        }
    }
}
