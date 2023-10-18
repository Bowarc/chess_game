// pub const IMAGE_TOKEN: &str = "$";
// Do we want to do "Start of text $ end of text", image_id1
// or "Start of text {image_id1} end of text" ?
// I think the first one is the best option
// Option we use precomputed data as computing it each frame is useless, so we'll use vec of parts, instead of parsin the raw_string each frame



// I'd like to have a way to set a color per part, having different size would fuck up lot of things tho



pub struct Text {
    id: crate::ui::Id,
    position: crate::ui::Position,
    // About the size, how do we make it fit as it's a text, do w use the total text nen / size.x?
    // If so, how do we manage the image ? i mean, spacing, imagr size, etc..
    req_size: crate::ui::Value,
    real_size: ggez::mint::Point2<crate::ui::Value>,
    style: crate::ui::Style,
    parts: Vec<TextPart>
}
    
pub enum TextPart{
    Text{
        raw: String, 
        color_opt: Option<crate::render::Color>
    },
    Image(crate::assets::sprite::SpriteId)
}

impl Text {
    pub fn new(
        position: crate::ui::Position,
        req_size: crate::ui::Value,
        style: crate::ui::Style,
        parts: Vec<TextPart>
    ) -> Self {
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

        // draw background
        if let Some(bg) = self.style.get_bg() {
            back_mesh.rectangle(
                ggez::graphics::DrawMode::fill(),
                self.get_computed_rect(ctx).into(),
                (*bg.get_color()).into(),
            )?;
        }
        // draw border
        if let Some(border) = self.style.get_border() {
            let r = shared::maths::Rect::new(
                real_rect.r_topleft() - border.get_size() / 2.,
                real_rect.size() + *border.get_size(),
                real_rect.rotation(),
            );

            front_mesh.rectangle(
                ggez::graphics::DrawMode::stroke(*border.get_size() as f32),
                r.into(),
                (*border.get_color()).into(),
            )?;
        };

        if self.parts.iter().map(|p| if let TextPart::Image(_) = p{0}else{1}).collect::<Vec<i32>>().iter().sum::<i32>() >0{
            // There is no image in the text, se we can optimise it by removing the maths to space images
            let mut global_text = ggez::graphics::Text::new("");
            global_text.set_layout(ggez::graphics::TextLayout::center());
            global_text.set_scale(self.req_size.compute(ctx) as f32);
            for (i, part) in self.parts.iter().enumerate(){
                let frag = match part{
                    TextPart::Text { raw, color_opt } =>{
                        let mut f = ggez::graphics::TextFragment::new(raw);
                        f.color = color_opt.map(|c| c.into());
                        
                        f
                    },
                    TextPart::Image(sprite_id) => unreachable!(),
                };
                global_text.add(frag);
            }
            let total_size: shared::maths::Vec2 = global_text.dimensions(ctx).unwrap().size().into();
            self.real_size = ggez::mint::Point2::from([total_size.x.into(), total_size.y.into()]);

            render_request.add(global_text, crate::render::DrawParam::default().pos(real_rect.center()), crate::render::Layer::Ui);
        }else{
            
            // let mut cursor = 0;


            error!("not handled yet")
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


impl TextPart{
    pub fn new_text(raw: String, color_opt: Option<crate::render::Color>) -> Self{
        Self::Text { raw, color_opt }
    }
    pub fn new_img(sprite_id: crate::assets::sprite::SpriteId) -> Self{
        Self::Image(sprite_id)
    }
}