pub struct TextEdit {
    id: crate::ui::Id,
    position: crate::ui::Position,
    width: crate::ui::Value,
    rows: usize, // Number of rows
    font_size: f64,
    size: ggez::mint::Point2<crate::ui::Value>,
    state: crate::ui::State,
    style: crate::ui::style::Bundle,
    txt: String,
}

impl TextEdit {
    pub fn new(
        position: crate::ui::Position,
        width: crate::ui::Value,
        rows: usize,
        font_size: f64,
        style: crate::ui::style::Bundle,
    ) -> Self {
        Self {
            id: crate::ui::Id::new(),
            position,
            width,
            rows,
            font_size,
            size: ggez::mint::Point2::from([0f64.into(), 0f64.into()]),
            state: crate::ui::State::default(),
            style,
            txt: String::new(),
        }
    }
}

impl super::TElement for TextEdit {
    fn draw(
        &mut self,
        ctx: &mut ggez::Context,
        back_mesh: &mut ggez::graphics::MeshBuilder,
        ui_mesh: &mut ggez::graphics::MeshBuilder,
        front_mesh: &mut ggez::graphics::MeshBuilder,
        render_request: &mut crate::render::RenderRequest,
    ) -> ggez::GameResult {
        let style = self.style.get(&self.state);
        let rect = self.get_computed_rect(ctx);

        if let Some(border) = style.get_border() {
            if let Err(e) = border.draw(front_mesh, rect) {
                error!("Error in text edit {} border draw {e}", self.get_id());
            }
        }

        if let Some(background) = style.get_bg() {
            if let Err(e) = background.draw(back_mesh, render_request, rect) {
                error!("Error in text edit {} background draw {e}", self.get_id());
            }
        }

        let txt = if self.state.focussed() {
            format!("{}|", self.txt)
        } else {
            self.txt.clone()
        };

        let text = ggez::graphics::Text::new(
            ggez::graphics::TextFragment::new(txt).scale(self.font_size as f32),
        );

        render_request.add(
            text,
            crate::render::DrawParam::default().rect(rect),
            crate::render::Layer::Ui,
        );

        self.size = ggez::mint::Point2::from_slice(&[
            self.width.clone(),
            crate::ui::Value::from(self.font_size * self.rows as f64),
        ]);
        Ok(())
    }

    fn get_size_value(&self) -> &ggez::mint::Point2<crate::ui::Value> {
        &self.size
    }

    fn get_pos_value(&self) -> &crate::ui::Position {
        &self.position
    }

    fn get_id(&self) -> crate::ui::Id {
        self.id
    }

    fn on_new_frame(&mut self) {
        self.state.new_frame()
    }
    fn on_mouse_press(
        &mut self,
        button: ggez::input::mouse::MouseButton,
        position: shared::maths::Point,
        ctx: &mut ggez::Context,
    ) {
        if shared::maths::collision::point_rect(position, self.get_computed_rect(ctx)) {
            self.state.mouse_press_self()
        } else {
            self.state.mouse_press_not_self()
        }
    }
    fn on_mouse_release(
        &mut self,
        button: ggez::input::mouse::MouseButton,
        position: shared::maths::Point,
        ctx: &mut ggez::Context,
    ) {
        if shared::maths::collision::point_rect(position, self.get_computed_rect(ctx)) {
            self.state.mouse_release_self()
        } else {
            self.state.mouse_release_not_self()
        }
    }
    fn on_mouse_motion(
        &mut self,
        position: shared::maths::Point,
        delta: shared::maths::Point,
        ctx: &mut ggez::Context,
    ) {
        if shared::maths::collision::point_rect(position, self.get_computed_rect(ctx)) {
            self.state.mouse_hover_self()
        } else {
            self.state.mouse_hover_not_self()
        }
    }

    fn on_text_input(&mut self, character: char, ctx: &mut ggez::Context) {
        if !self.state.focussed() {
            return;
        }
        // https://en.wikipedia.org/wiki/List_of_Unicode_characters
        match character{
            '\u{20}'            | /* space character */
            '\u{21}'..='\u{2f}' | /* !"#$%&'()*+,-./ */ 
            '\u{30}'..='\u{39}' | /* 0123456789 */ 
            '\u{3A}'..='\u{40}' | /* :;<=>?@ */ 
            '\u{41}'..='\u{5A}' | /* ABCDEFGHIJKLMNOPQRSTUVWXYZ */ 
            '\u{5B}'..='\u{60}' | /* [\]^_` */ 
            '\u{61}'..='\u{7A}' | /* abcdefghijklmnopqrstuvwxy */ 
            '\u{7B}'..='\u{7E}'   /* {|}~ */ => {
                self.txt.push(character)
            },
            '\u{d}' | '\u{a}' /* New line caracterS */ => {
                // I don't like having to check for both but i have to
                let new_line_count = self.txt.matches('\u{a}').count() + self.txt.matches('\u{d}').count();

                // debug!("{new_line_count} | {}", self.rows -1);
                if new_line_count < self.rows -1{
                    self.txt.push(character)
                }
            },
            '\u{8}' /* Delete */ => {
                self.txt.pop();
            },
            _ => {
                warn!("unhandled character: '{character}', '{}'", character.escape_unicode())
            }
        }
    }
}
