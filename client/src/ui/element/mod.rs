mod button;
mod default;
mod graph;

use button::Button;
use graph::Graph;

#[enum_dispatch::enum_dispatch(TElement)]
pub enum Element {
    Button,
    Graph,
}
#[enum_dispatch::enum_dispatch]
pub trait TElement {
    fn get_base(&self) -> &ElementBase;
    fn get_base_mut(&mut self) -> &mut ElementBase;
    fn draw(
        &mut self,
        _: &mut ggez::Context,
        _back: &mut ggez::graphics::MeshBuilder,
        _ui: &mut ggez::graphics::MeshBuilder,
        _front: &mut ggez::graphics::MeshBuilder,
        _: &mut crate::render::RenderRequest,
    ) -> ggez::GameResult;

    /*
        ↑
        Required


        Auto impls
        ↓
    */

    fn get_id(&self) -> &super::Id {
        &self.get_base().id
    }
    fn get_state(&self) -> &super::State {
        &self.get_base().state
    }
    fn get_state_mut(&mut self) -> &mut super::State {
        &mut self.get_base_mut().state
    }
    fn get_style_bundle(&self) -> &super::style::Bundle {
        &self.get_base().style
    }
    fn get_style_bundle_mut(&mut self) -> &mut super::style::Bundle {
        &mut self.get_base_mut().style
    }
    fn get_size_value(&self) -> &ggez::mint::Point2<crate::ui::Value> {
        &self.get_base().size
    }
    fn get_computed_size(&self, ctx: &mut ggez::Context) -> shared::maths::Vec2 {
        let sizev = self.get_size_value();

        shared::maths::Point::new(sizev.x.compute(ctx), sizev.y.compute(ctx))
    }
    fn get_pos_value(&self) -> &super::Position {
        &self.get_base().position
    }
    fn get_computed_pos(
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
    fn get_computed_rect(&self, ctx: &mut ggez::Context) -> shared::maths::Rect {
        let size = self.get_computed_size(ctx);

        let position = self.get_computed_pos(ctx, Some(size));

        shared::maths::Rect::new(position, size, 0.)
    }

    /*
        Events
    */

    fn on_mouse_press(
        &mut self,
        button: ggez::input::mouse::MouseButton,
        position: shared::maths::Point,
        ctx: &mut ggez::Context,
    ) {
        let rect = self.get_computed_rect(ctx);
        let state = self.get_state_mut();
        if shared::maths::collision::point_rect(position, rect) {
            state.mouse_press_self()
        } else {
            state.mouse_press_not_self()
        }
    }
    fn on_mouse_release(
        &mut self,
        button: ggez::input::mouse::MouseButton,
        position: shared::maths::Point,
        ctx: &mut ggez::Context,
    ) {
        let rect = self.get_computed_rect(ctx);
        let state = self.get_state_mut();
        if shared::maths::collision::point_rect(position, rect) {
            state.mouse_release_self()
        } else {
            state.mouse_release_not_self()
        }
    }
    fn on_mouse_motion(
        &mut self,
        pos: shared::maths::Point,
        delta: shared::maths::Point,
        ctx: &mut ggez::Context,
    ) {
        let rect = self.get_computed_rect(ctx);
        let state = self.get_state_mut();
        if shared::maths::collision::point_rect(pos, rect) {
            state.mouse_hover_self()
        } else {
            state.mouse_hover_not_self()
        }
    }
    fn on_mouse_wheel(&mut self, _delta: shared::maths::Point, _ctx: &mut ggez::Context) {
        // Idk
    }
    fn on_key_down(
        &mut self,
        input: ggez::input::keyboard::KeyInput,
        repeated: bool,
        ctx: &mut ggez::Context,
    ) {
        // Idk
    }
    fn on_key_up(&mut self, _input: ggez::input::keyboard::KeyInput, _ctx: &mut ggez::Context) {
        // Idk
    }
    fn on_text_input(&mut self, _character: char, _ctx: &mut ggez::Context) {
        // Idk
    }
}

pub struct ElementBase {
    pub id: super::Id,
    pub position: super::Position,
    pub size: ggez::mint::Point2<crate::ui::Value>,
    pub state: super::State,
    pub style: super::style::Bundle,
}

impl Element {
    pub fn new_button(base: ElementBase) -> Self {
        Self::Button(button::Button::new(base))
    }
}

impl ElementBase {
    pub fn new(
        position: super::Position,
        size: (impl Into<super::Value>, impl Into<super::Value>),
        style: super::style::Bundle,
    ) -> Self {
        let x = size.0.into();
        let y = size.1.into();
        let size = ggez::mint::Point2::from([x, y]);
        ElementBase {
            id: super::Id::new(),
            position,
            size,
            state: super::State::default(),
            style,
        }
    }
}
