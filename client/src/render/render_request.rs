pub struct RenderRequest {
    inner: hashbrown::HashMap<super::Layer, Vec<(RenderRequestBit, super::DrawParam)>>,
}

#[derive(Clone, Debug)]
pub enum RenderRequestBit {
    Sprite(crate::assets::sprite::SpriteId),
    Mesh(ggez::graphics::Mesh),
    MeshBuilder(ggez::graphics::MeshBuilder),
    Text(ggez::graphics::Text),
    EguiWindow,
}

impl RenderRequest {
    pub fn new() -> Self {
        Self {
            inner: hashbrown::HashMap::with_capacity(10),
        }
    }

    pub fn add<T: Into<RenderRequestBit>>(
        &mut self,
        render_request_bit: T,
        dp: super::DrawParam,
        layer: super::Layer,
    ) {
        self.inner
            .entry(layer)
            .or_insert_with(|| Vec::with_capacity(50))
            .push((render_request_bit.into(), dp))
    }

    pub fn add_batch<T: Into<RenderRequestBit>>(
        &mut self,
        batch: Vec<(T, super::DrawParam, super::Layer)>,
    ) {
        for (render_request_bit, dp, layer) in batch {
            self.add(render_request_bit, dp, layer)
        }
    }

    pub fn clear(&mut self) {
        self.inner.clear()
    }
}

// Not used anymore but was good so im keeping it
// fn get_order_value_from_drawable(
//     d: &impl ggez::graphics::Drawable,
//     ctx: &mut ggez::Context,
// ) -> i32 {
//     match d.dimensions(ctx) {
//         Some(d) => (d.y + (d.h * 0.5)) as i32,
//         None => {
//             error!("Could not get dimensions of Drawable");
//             0
//         }
//     }
// }

impl std::ops::Deref for RenderRequest {
    type Target = hashbrown::HashMap<super::Layer, Vec<(RenderRequestBit, super::DrawParam)>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RenderRequest {
    // type Target = HashMap<Layer, Vec<RenderRequestBit>>;

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl From<crate::assets::sprite::SpriteId> for RenderRequestBit {
    fn from(sprite: crate::assets::sprite::SpriteId) -> RenderRequestBit {
        RenderRequestBit::Sprite(sprite)
    }
}

impl From<ggez::graphics::Mesh> for RenderRequestBit {
    fn from(mesh: ggez::graphics::Mesh) -> RenderRequestBit {
        RenderRequestBit::Mesh(mesh)
    }
}

impl From<ggez::graphics::MeshBuilder> for RenderRequestBit {
    fn from(mesh: ggez::graphics::MeshBuilder) -> RenderRequestBit {
        RenderRequestBit::MeshBuilder(mesh)
    }
}

impl From<ggez::graphics::Text> for RenderRequestBit {
    fn from(text: ggez::graphics::Text) -> RenderRequestBit {
        RenderRequestBit::Text(text)
    }
}
