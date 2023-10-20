// pub const IMAGE_TOKEN: &str = "$";
// Do we want to do "Start of text $ end of text", image_id1
// or "Start of text {image_id1} end of text" ?
// I think the first one is the best option
// Option we use precomputed data as computing it each frame is useless, so we'll use vec of parts, instead of parsing the raw_string each frame

// I'd like to have a way to set a color per part, having different size would fuck up lot of things tho

pub struct Text {
    id: crate::ui::Id,
    position: crate::ui::Position,
    // About the size, how do we make it fit as it's a text, do w use the total text len / size.x?
    // If so, how do we manage the image ? i mean, spacing, image size, etc..
    req_size: crate::ui::Value,
    real_size: ggez::mint::Point2<crate::ui::Value>,
    style: crate::ui::Style,
    parts: Vec<Vec<TextPart>>,
}

#[derive(Clone)]
#[derive(Debug)]
pub enum TextPart {
    Text {
        raw: String,
        color_opt: Option<crate::render::Color>,
    },
    Image(crate::assets::sprite::SpriteId),
}

enum ComputedTextPart{
    Text{
        ggtext: ggez::graphics::Text,
    }
}

impl Text {
    pub fn new(
        position: crate::ui::Position,
        req_size: crate::ui::Value,
        style: crate::ui::Style,
        user_parts: Vec<TextPart>,
    ) -> Self {
        let mut parts = Vec::<Vec<TextPart>>::default();
        // Build parts
        let mut row = Vec::new();
        for part in user_parts{
            let need_new = if let TextPart::Text{ raw, .. } = &part{
                raw.contains('\n')
            }else{  
                false
            };

            row.push(part);
            if need_new{
                parts.push(row.clone());
                row.clear();
            }
        }
        Self {
            id: crate::ui::Id::new(),
            position,
            req_size,
            real_size: ggez::mint::Point2::from([0f64.into(), 0f64.into()]),
            style,
            parts,
        }
    }
}

impl super::TElement for Text {
    fn draw(
        &mut self,
        ctx: &mut ggez::Context,
        back_mesh: &mut ggez::graphics::MeshBuilder,
        ui_mesh: &mut ggez::graphics::MeshBuilder,
        front_mesh: &mut ggez::graphics::MeshBuilder,
        render_request: &mut crate::render::RenderRequest,
    ) -> ggez::GameResult {
        use ggez::graphics::Drawable as _;
        let real_rect = self.get_computed_rect(ctx);
        let target_size = self.req_size.compute(ctx);

        // draw background
        if let Some(bg) = self.style.get_bg() {
            bg.draw(back_mesh, render_request, real_rect)?
        }

        // draw border
        if let Some(border) = self.style.get_border() {
            border.draw(front_mesh, real_rect)?;
        };


        for row in &self.parts{
            let len = row.len();    
            for (i, part) in row.iter().enumerate(){


            }
        }


        
        Ok(())
    }
    fn get_size_value(&self) -> &ggez::mint::Point2<crate::ui::Value> {
        &self.real_size
    }
    fn get_pos_value(&self) -> &crate::ui::Position {
        &self.position
    }
    fn get_id(&self) -> shared::id::Id {
        self.id
    }
}

impl TextPart {
    pub fn new_text(raw: String, color_opt: Option<crate::render::Color>) -> Self {
        Self::Text { raw, color_opt }
    }
    pub fn new_img(sprite_id: crate::assets::sprite::SpriteId) -> Self {
        Self::Image(sprite_id)
    }
}
