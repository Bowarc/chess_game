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
    bits: Vec<TextBit>,
}

#[derive(Clone, Debug)]
pub enum TextBit{
    Text {
        raw: String,
        color_opt: Option<crate::render::Color>,
    },
    Image(crate::assets::sprite::SpriteId),
}

#[derive(Clone)]
enum ComputedTextBit{
    Text(ggez::graphics::Text),
    Image(crate::assets::sprite::SpriteId)
}


impl Text {
    pub fn new(
        position: crate::ui::Position,
        req_size: crate::ui::Value,
        style: crate::ui::Style,
        bits: Vec<TextBit>,
    ) -> Self {
        // Here we split the texts when we find multiple new lines in a single bit
        // but what if a bit as a lot of new lines chars ?
        // well it's up to the user to not do stoopid sht
        // jk i'll find a cool solution later
        // TODO ^

        // let mut new_bits = Vec::new();

        for bit in bits.iter(){

        }

        Self {
            id: crate::ui::Id::new(),
            position,
            req_size,
            real_size: ggez::mint::Point2::from([0f64.into(), 0f64.into()]),
            style,
            bits,
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


        let mut draw_curr_row = |curr_row:  Vec<ComputedTextBit>, curr_width: f64, curr_height: f64|{
            let mut x = 0.;
            for computed_bit in curr_row{
                match computed_bit{
                    ComputedTextBit::Text(ggtext) =>{
                        let w = ggtext.dimensions(ctx).unwrap().w;
                        render_request.add(
                            ggtext, 
                            crate::render::DrawParam::default()
                                .pos(real_rect.center() +
                                    shared::maths::Point::new(
                                        x - curr_width *0.5, 
                                        0.  + curr_height - real_rect.height() * 0.5
                                    )
                                ),
                            crate::render::Layer::Ui
                        );
                        x += w as f64;
                    },
                    ComputedTextBit::Image(sprite_id) => {
                        render_request.add(
                            sprite_id,
                             crate::render::DrawParam::default()
                                .pos(
                                    real_rect.center() + 
                                    shared::maths::Point::new(
                                        x - curr_width * 0.5,
                                        0. + curr_height - real_rect.height()* 0.5
                                    ) + 
                                    target_size * 0.4
                                )
                                .size(target_size),
                            crate::render::Layer::Ui
                        );
                        x += target_size;
                    },
                }
            }
        };

        let mut total_size = shared::maths::Vec2::ZERO;

        let mut curr_row = Vec::new();
        let mut curr_width = 0.;
        let mut curr_height = 0.;
        for (i,bit) in self.bits.iter().enumerate(){
            match bit{
                TextBit::Text { raw, color_opt } => {
                    let nlc = raw.matches('\n').count();
                    let mut f = ggez::graphics::TextFragment::new(raw);
                    f.color = color_opt.map(|c| c.into());
                    let ggtext = ggez::graphics::Text::new(f);
                    curr_width += ggtext.dimensions(ctx).unwrap().w as f64;
                    curr_row.push(ComputedTextBit::Text(ggtext));


                    if nlc > 0 || i == self.bits.len()- 1{
                        // Draw then increment the Y position and reset the x position
                        draw_curr_row(curr_row, curr_width, curr_height);
                        curr_row = Vec::new();

                        if curr_width > total_size.x{
                            total_size.x = curr_width;
                        }
                        curr_width = 0.;
                        curr_height += target_size;
                    }
                },
                TextBit::Image(sprite_id) => {
                    curr_row.push(
                        ComputedTextBit::Image(*sprite_id)
                    );
                    curr_width += target_size;
                }
            }
        }        
        // draw_curr_row(curr_row, curr_width, curr_height);
        // curr_height += target_size;
        total_size.y = curr_height;

        debug!("{total_size:?}");

        self.real_size = ggez::mint::Point2::from([crate::ui::Value::fixed(total_size.x), crate::ui::Value::fixed(total_size.y)]);
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

impl TextBit{
    pub fn new_text(raw: String, color_opt: Option<crate::render::Color>) -> Self {
        Self::Text { raw, color_opt }
    }
    pub fn new_img(sprite_id: crate::assets::sprite::SpriteId) -> Self {
        Self::Image(sprite_id)
    }
}
