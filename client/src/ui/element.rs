pub struct Element {
    widget: super::widget::Widget,
    id: super::Id,
    position: super::Position,
    size: ggez::mint::Point2<crate::ui::Value>,
    state: super::State,
    style: super::style::Bundle,
}

/// Constructors
impl Element {
    fn new(
        given_widget: super::widget::Widget,
        position: super::Position,
        size: (impl Into<super::Value>, impl Into<super::Value>),
        style: super::style::Bundle,
    ) -> Self {
        let x = size.0.into();
        let y = size.1.into();
        let size = ggez::mint::Point2::from([x, y]);
        Self {
            widget: given_widget,
            id: super::Id::new(),
            position,
            size,
            state: super::State::default(),
            style,
        }
    }

    pub fn new_button(
        position: super::Position,
        size: (impl Into<super::Value>, impl Into<super::Value>),
        style: super::style::Bundle,
    ) -> Self {
        Self::new(super::widget::Widget::new_button(), position, size, style)
    }
}

/// Others
impl Element {
    pub fn draw(
        &mut self,
        _: &mut ggez::Context,
        _back: &mut ggez::graphics::MeshBuilder,
        _ui: &mut ggez::graphics::MeshBuilder,
        _front: &mut ggez::graphics::MeshBuilder,
        _: &mut crate::render::RenderRequest,
    ) -> ggez::GameResult {
        // self.widget.draw()

        Ok(())
    }
}

/// Getters
impl Element {
    pub fn get_id(&self) -> &super::Id {
        &self.id
    }
    pub fn get_state(&self) -> &super::State {
        &self.state
    }
    pub fn get_state_mut(&mut self) -> &mut super::State {
        &mut self.state
    }
    pub fn get_style_bundle(&self) -> &super::style::Bundle {
        &self.style
    }
    pub fn get_style_bundle_mut(&mut self) -> &mut super::style::Bundle {
        &mut self.style
    }
    pub fn get_size_value(&self) -> &ggez::mint::Point2<crate::ui::Value> {
        &self.size
    }
    pub fn get_computed_size(&self, ctx: &mut ggez::Context) -> shared::maths::Vec2 {
        let sizev = self.get_size_value();

        shared::maths::Point::new(sizev.x.compute(ctx), sizev.y.compute(ctx))
    }
    pub fn get_pos_value(&self) -> &super::Position {
        &self.position
    }
    pub fn get_computed_pos(
        &self,
        ctx: &mut ggez::Context,
        size_opt: Option<shared::maths::Vec2>,
    ) -> shared::maths::Point {
        let posv = self.get_pos_value();

        let size = if let Some(size) = size_opt {
            size
        } else {
            self.get_computed_size(ctx)
        };
        posv.compute(ctx, size)
    }
    pub fn get_computed_rect(&self, ctx: &mut ggez::Context) -> shared::maths::Rect {
        let size = self.get_computed_size(ctx);

        let position = self.get_computed_pos(ctx, Some(size));

        shared::maths::Rect::new(position, size, 0.)
    }
}

/// Events
impl Element {
    pub fn on_mouse_press(
        &mut self,
        button: ggez::input::mouse::MouseButton,
        position: shared::maths::Point,
        ctx: &mut ggez::Context,
    ) {
        let rect = self.get_computed_rect(ctx);
        if shared::maths::collision::point_rect(position, rect) {
            self.state.mouse_press_self()
        } else {
            self.state.mouse_press_not_self()
        }
    }
    pub fn on_mouse_release(
        &mut self,
        button: ggez::input::mouse::MouseButton,
        position: shared::maths::Point,
        ctx: &mut ggez::Context,
    ) {
        let rect = self.get_computed_rect(ctx);

        if shared::maths::collision::point_rect(position, rect) {
            self.state.mouse_release_self()
        } else {
            self.state.mouse_release_not_self()
        }
    }
    pub fn on_mouse_motion(
        &mut self,
        pos: shared::maths::Point,
        delta: shared::maths::Point,
        ctx: &mut ggez::Context,
    ) {
        let rect = self.get_computed_rect(ctx);
        if shared::maths::collision::point_rect(pos, rect) {
            self.state.mouse_hover_self()
        } else {
            self.state.mouse_hover_not_self()
        }
    }
    pub fn on_mouse_wheel(&mut self, _delta: shared::maths::Point, _ctx: &mut ggez::Context) {
        // Idk
    }
    pub fn on_key_down(
        &mut self,
        input: ggez::input::keyboard::KeyInput,
        repeated: bool,
        ctx: &mut ggez::Context,
    ) {
        // Idk
    }
    pub fn on_key_up(&mut self, _input: ggez::input::keyboard::KeyInput, _ctx: &mut ggez::Context) {
        // Idk
    }
    pub fn on_text_input(&mut self, _character: char, _ctx: &mut ggez::Context) {
        // Idk
    }
}
