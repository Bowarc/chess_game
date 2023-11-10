pub struct Button {
    id: crate::ui::Id,
    position: crate::ui::Position,
    size: ggez::mint::Point2<crate::ui::Value>,
    state: crate::ui::State,
    style: crate::ui::style::Bundle,
}

impl Button {
    pub fn new(
        id: crate::ui::Id,
        position: crate::ui::Position,
        size: ggez::mint::Point2<crate::ui::Value>,
        style: crate::ui::style::Bundle,
    ) -> Self {
        Self {
            id,
            position,
            size,
            state: crate::ui::State::default(),
            style,
        }
    }

    pub fn clicked_this_frame(&self) -> bool {
        self.state.clicked_this_frame()
    }

    fn get_correct_style(&self) -> &crate::ui::style::Bundle {
        &self.style
    }
}

impl super::TElement for Button {
    fn draw(
        &mut self,
        ctx: &mut ggez::Context,
        back_mesh: &mut ggez::graphics::MeshBuilder,
        ui_mesh: &mut ggez::graphics::MeshBuilder,
        front_mesh: &mut ggez::graphics::MeshBuilder,
        render_request: &mut crate::render::RenderRequest,
    ) -> ggez::GameResult {
        let rect = self.get_computed_rect(ctx);
        let style = self.style.get(&self.state);

        // draw background
        if let Some(bg) = style.get_bg() {
            bg.draw(back_mesh, render_request, rect)?
        }

        // draw border
        if let Some(border) = style.get_border() {
            border.draw(front_mesh, rect)?;
        };

        ui_mesh.rectangle(
            ggez::graphics::DrawMode::fill(),
            rect.into(),
            (*style.get_color()).into(),
        )?;
        Ok(())
    }
    fn get_size_value(&self) -> &ggez::mint::Point2<crate::ui::Value> {
        &self.size
    }
    fn get_pos_value(&self) -> &crate::ui::Position {
        &self.position
    }
    fn get_id(&self) -> crate::ui::Id {
        self.id.clone()
    }

    fn on_new_frame(&mut self) {
        self.state.new_frame();
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
}
