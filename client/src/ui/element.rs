mod button;

pub struct Element {
    pub t: ElementType,
    pub id: super::Id,
    pub position: super::Position,
    pub size: ggez::mint::Point2<crate::ui::Value>,
    pub state: super::State,
    pub style: super::style::Bundle,
}

pub enum ElementType {
    Button(button::Button),
}

impl ElementType {
    pub fn new_button() -> Self {
        Self::Button(button::Button {})
    }
}

impl Element {
    pub fn new(
        t: ElementType,
        position: super::Position,
        size: (impl Into<super::Value>, impl Into<super::Value>),
        style: super::style::Bundle,
    ) -> Self {
        let x = size.0.into();
        let y = size.1.into();
        let size = ggez::mint::Point2::from([x, y]);
        Element {
            t,
            id: String::from("This is a test"),
            position,
            size,
            state: super::State::default(),
            style,
        }
    }
    pub fn compute_rect(&self, ctx: &mut ggez::Context) -> shared::maths::Rect {
        let size = shared::maths::Point::new(self.size.x.compute(ctx), self.size.y.compute(ctx));

        let position = self.position.compute(ctx, size);

        shared::maths::Rect::new(position, size, 0.)
    }

    pub fn draw(
        &mut self,
        ctx: &mut ggez::Context,
        global_mesh: &mut ggez::graphics::MeshBuilder,
        top_mesh: &mut ggez::graphics::MeshBuilder,
        _render_request: &mut crate::render::RenderRequest,
    ) -> ggez::GameResult {
        let rect = self.compute_rect(ctx);
        let style = self.style.get(&self.state);

        if let Some(border) = style.get_border() {
            let r = shared::maths::Rect::new(
                rect.r_topleft() - border.get_size() / 2.,
                rect.size() + *border.get_size(),
                rect.rotation(),
            );

            // Draw the border above the square
            top_mesh.rectangle(
                ggez::graphics::DrawMode::stroke(*border.get_size() as f32),
                r.into(),
                (*border.get_color()).into(),
            )?;
        };

        match &self.t {
            ElementType::Button(_btn) => {
                global_mesh.rectangle(
                    ggez::graphics::DrawMode::fill(),
                    rect.into(),
                    (*style.get_color()).into(),
                )?;
            }
        }
        Ok(())
    }
}

/// Event implementation
impl Element {
    pub fn on_mouse_press(
        &mut self,
        button: ggez::input::mouse::MouseButton,
        position: shared::maths::Point,
    ) {
    }
    pub fn on_mouse_release(
        &mut self,
        button: ggez::input::mouse::MouseButton,
        position: shared::maths::Point,
    ) {
    }
    pub fn on_mouse_wheel(&mut self, delta: shared::maths::Point) {}
    pub fn on_key_down(&mut self, input: ggez::input::keyboard::KeyInput, repeated: bool) {}
    pub fn on_key_up(&mut self, input: ggez::input::keyboard::KeyInput) {}
    pub fn on_text_input(&mut self, character: char) {}
}
