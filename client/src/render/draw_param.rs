#[derive(Debug, Clone, Copy)]
pub struct DrawParam {
    pub pos: shared::maths::Vec2,
    pub size: Option<shared::maths::Vec2>,
    pub scale: shared::maths::Vec2,
    pub offset: shared::maths::Vec2,
    pub rotation: f64,
    pub color: super::Color,
}

#[allow(dead_code)]
impl DrawParam {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn pos(mut self, pos: impl Into<shared::maths::Vec2>) -> Self {
        self.pos = pos.into();
        self
    }
    pub fn dest(mut self, dest: impl Into<shared::maths::Vec2>) -> Self {
        self.pos = dest.into();
        self
    }
    pub fn size(mut self, size: impl Into<shared::maths::Vec2>) -> Self {
        self.size = Some(size.into());
        self
    }
    pub fn scale(mut self, scale: impl Into<shared::maths::Vec2>) -> Self {
        self.scale = scale.into();
        self
    }
    pub fn offset<T: Into<shared::maths::Vec2>>(mut self, offset: T) -> Self {
        self.offset = offset.into();
        self
    }
    pub fn rotation(mut self, rotation: f64) -> Self {
        self.rotation = rotation;
        self
    }
    pub fn color(mut self, color: impl Into<super::Color>) -> Self {
        self.color = color.into();
        self
    }

    pub fn rect(mut self, rect: shared::maths::Rect) -> Self {
        self.pos = rect.aa_topleft();
        self.size = Some(rect.size());
        self
    }
    pub fn to_ggez_scaled(
        self,
        image_size: impl Into<shared::maths::Point>,
    ) -> ggez::graphics::DrawParam {
        if let Some(size) = self.size {
            let image_size = image_size.into();
            ggez::graphics::DrawParam::new()
                .dest(self.pos)
                .scale((size / image_size) * self.scale)
                .offset(image_size / shared::maths::Point::new(2., 2.) + self.offset)
                .rotation(self.rotation as f32)
                .color(self.color)
        } else {
            self.to_ggez_unscaled()
        }
    }
    pub fn to_ggez_unscaled(self) -> ggez::graphics::DrawParam {
        ggez::graphics::DrawParam::new()
            .dest(self.pos)
            .scale(self.scale)
            .offset(self.offset)
            .rotation(self.rotation as f32)
            .color(self.color)
    }
}

impl std::default::Default for DrawParam {
    fn default() -> Self {
        Self {
            pos: shared::maths::Vec2::ZERO,
            size: None,
            scale: shared::maths::Vec2::ONE,
            offset: shared::maths::Vec2::ZERO,
            rotation: 0.,
            color: [255, 255, 255, 255].into(),
        }
    }
}

impl From<DrawParam> for ggez::graphics::DrawParam {
    fn from(val: DrawParam) -> Self {
        val.to_ggez_unscaled()
    }
}
