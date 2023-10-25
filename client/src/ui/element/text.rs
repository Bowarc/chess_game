use ggez::graphics::Drawable;

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
    NewLine,
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
        let mut new_bits = Vec::new();

        for bit in bits{
            match &bit{
                TextBit::Text { raw, color_opt } => {
                    if raw.contains('\n'){
                        let raws = raw.split('\n').collect::<Vec<&str>>();
                        for (i,splitted) in raws.iter().enumerate(){
                            new_bits.push(
                                TextBit::Text { raw: splitted.to_string(), color_opt: *color_opt }
                            );   
                            if i < raws.len() - 1{
                                new_bits.push(
                                    TextBit::NewLine
                                );   
                            }
                        }
                    }else{
                        new_bits.push(bit)
                    }
                },
                _ => {
                    new_bits.push(bit)
                }
            }
        }

        // Remove all empty strings
        let mut i = 0;
        while i < new_bits.len(){
            let mut remove= false;
            if let TextBit::Text { raw ,.. } = new_bits.get(i).unwrap(){
                if raw.is_empty(){
                    remove = true;
                }
            }

            if remove{
                new_bits.remove(i);
            }else{
                i+=1;
            }
        }

        // Do we pop if the last bit is a new line ?
        /*unsure */{
            while let Some(TextBit::NewLine) = new_bits.last(){
                new_bits.pop();
            }
        }

        debug!("{new_bits:?}");

        Self {
            id: crate::ui::Id::new(),
            position,
            req_size,
            real_size: ggez::mint::Point2::from([0f64.into(), 0f64.into()]),
            style,
            bits: new_bits,
        }
    }

    fn draw_bits_single_text(&mut self, 
        ctx: &mut ggez::Context,
        target_size: f64,
        real_rect: &shared::maths::Rect,
        render_request: &mut crate::render::RenderRequest,
    ){
        // This is called under the assumption that there is no image in the bits !

        let mut global_text = ggez::graphics::Text::new("");
        global_text.set_layout(ggez::graphics::TextLayout::center());
        for bit in self.bits.iter(){
            match bit{
                TextBit::Text { raw, color_opt } => {
                    let mut f = ggez::graphics::TextFragment::new(raw.clone()).scale(target_size as f32);
                    f.color = color_opt.map(|c| c.into());
                    global_text.add(f);
                },
                TextBit::NewLine => {
                    global_text.add('\n');
                }
                TextBit::Image(_) => unreachable!("You're not supposed to draw images in this loop"),
            }
        }
        let size = global_text.dimensions(ctx).unwrap().size();

        self.real_size = ggez::mint::Point2::from([crate::ui::Value::fixed(size.x.into()), crate::ui::Value::fixed(size.y.into())]);

        render_request.add(global_text, crate::render::DrawParam::default().pos(real_rect.center()), crate::render::Layer::Ui);
    }

    fn draw_bits_multi_text(&mut self,
        ctx: &mut ggez::Context,
        target_size: f64,
        real_rect: &shared::maths::Rect,
        render_request: &mut crate::render::RenderRequest
    ){
        use ggez::graphics::Drawable as _;
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
                                        x - curr_width * 0.5, 
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
                                        0. + curr_height - real_rect.height() * 0.5
                                    )
                                     +  shared::maths::Vec2::new(0.5, 0.4) * target_size
                                    // This is sort of fcked, koz when the row is a single image, the image is not centered
                                    // TODO fix ?
                                    // Hmm removing the last line doesn't fix the problem, i don't think it's that important rn
                                    // i'll still open an issue
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
            let mut need_draw = false;
            match bit{
                TextBit::Text { raw, color_opt } => {
                    let mut f = ggez::graphics::TextFragment::new(raw).scale(target_size as f32);
                    f.color = color_opt.map(|c| c.into());
                    let ggtext = ggez::graphics::Text::new(f);
                    curr_width += ggtext.dimensions(ctx).unwrap().w as f64;
                    curr_row.push(ComputedTextBit::Text(ggtext));
                },
                TextBit::Image(sprite_id) => {
                    curr_row.push(
                        ComputedTextBit::Image(*sprite_id)
                    );
                    curr_width += target_size;
                }
                TextBit::NewLine => {
                    need_draw = true;
                },
            }

            if need_draw || i == self.bits.len() - 1{
                draw_curr_row(curr_row, curr_width, curr_height);
                curr_row = Vec::new();

                if curr_width > total_size.x{
                    total_size.x = curr_width;
                }
                curr_width = 0.;
                curr_height += target_size;
            }
        }        
        total_size.y = curr_height;

        self.real_size = ggez::mint::Point2::from([crate::ui::Value::fixed(total_size.x), crate::ui::Value::fixed(total_size.y)]);

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


        let image_count: i32 = self.bits.iter().map(|bit|{
            if let TextBit::Image(_) = bit{
                1
            }else{0}
        }).sum();

        if image_count > 0{
            self.draw_bits_multi_text(ctx, target_size, &real_rect, render_request);
        }else{
            self.draw_bits_single_text(ctx, target_size, &real_rect, render_request);
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

impl TextBit{
    pub fn new_text(raw: String, color_opt: Option<crate::render::Color>) -> Self {
        Self::Text { raw, color_opt }
    }
    pub fn new_img(sprite_id: crate::assets::sprite::SpriteId) -> Self {
        Self::Image(sprite_id)
    }
}
