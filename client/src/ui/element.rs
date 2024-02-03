mod button;
mod graph;
mod text;
mod text_edit;
mod image;

pub use button::Button;
pub use graph::{Graph, GraphText};
pub use text::{Text, TextBit};
pub use text_edit::TextEdit;
pub use image::Image;

#[enum_dispatch::enum_dispatch(TElement)]
pub enum Element {
    Button,
    Graph,
    Text,
    TextEdit,
    Image
}

#[enum_dispatch::enum_dispatch]
pub trait TElement: std::any::Any {
    fn draw(
        &mut self,
        _: &mut ggez::Context,
        _back: &mut ggez::graphics::MeshBuilder,
        _ui: &mut ggez::graphics::MeshBuilder,
        _front: &mut ggez::graphics::MeshBuilder,
        _: &mut crate::render::RenderRequest,
    ) -> ggez::GameResult;

    fn get_size_value(&self) -> &super::Vector;

    fn get_pos_value(&self) -> &super::Position;

    fn get_id(&self) -> super::Id;

    /*
        ↑
        Required
        Auto impls
        ↓
    */

    fn get_computed_size(&self, ctx: &mut ggez::Context) -> shared::maths::Vec2 {
        let sizev = self.get_size_value();

        shared::maths::Point::new(sizev.x().compute(ctx), sizev.y().compute(ctx))
    }

    fn get_computed_pos(
        &self,
        ctx: &mut ggez::Context,
        size_opt: Option<shared::maths::Vec2>,
    ) -> shared::maths::Point {
        let posv = self.get_pos_value();

        let size = size_opt.unwrap_or_else(|| self.get_computed_size(ctx));

        posv.compute(ctx, size)
    }

    fn get_computed_rect(&self, ctx: &mut ggez::Context) -> shared::maths::Rect {
        let size = self.get_computed_size(ctx);

        let position = self.get_computed_pos(ctx, Some(size));

        shared::maths::Rect::new_from_center(position, size, 0.)
    }

    /*
        Events
    */

    fn on_mouse_press(
        &mut self,
        _button: ggez::input::mouse::MouseButton,
        _position: shared::maths::Point,
        _ctx: &mut ggez::Context,
    ) {
    }
    fn on_mouse_release(
        &mut self,
        _button: ggez::input::mouse::MouseButton,
        _position: shared::maths::Point,
        _ctx: &mut ggez::Context,
    ) {
    }
    fn on_mouse_motion(
        &mut self,
        _position: shared::maths::Point,
        _delta: shared::maths::Point,
        _ctx: &mut ggez::Context,
    ) {
    }
    fn on_mouse_wheel(&mut self, _delta: shared::maths::Point, _ctx: &mut ggez::Context) {}
    fn on_key_down(
        &mut self,
        _key: ggez::input::keyboard::KeyInput,
        _repeated: bool,
        _ctx: &mut ggez::Context,
    ) {
    }
    fn on_key_up(&mut self, _key: ggez::input::keyboard::KeyInput, _ctx: &mut ggez::Context) {}
    fn on_text_input(&mut self, _character: char, _ctx: &mut ggez::Context) {}
    fn on_new_frame(&mut self) {}
}

/// Constructors
impl Element {
    pub fn new_button(
        id: impl Into<super::Id>,
        position: impl Into<super::Position>, // Center
        size: impl Into<super::Vector>,
        style: super::style::Bundle,
    ) -> Self {
        Self::Button(button::Button::new(
            id.into(),
            position.into(),
            size.into(),
            style,
        ))
    }
    pub fn new_graph(
        id: impl Into<super::Id>,
        position: impl Into<super::Position>, // Center
        size: impl Into<super::Vector>,
        style: super::Style,
        text: Option<graph::GraphText>,
    ) -> Self {
        Self::Graph(graph::Graph::new(
            id.into(),
            position.into(),
            size.into(),
            style,
            text,
        ))
    }
    pub fn new_text(
        id: impl Into<super::Id>,
        position: impl Into<super::Position>, // Center
        size: impl Into<super::Value>,
        style: super::Style,
        parts: Vec<TextBit>,
    ) -> Self {
        let size = size.into();
        Self::Text(Text::new(id.into(), position.into(), size, style, parts))
    }
    pub fn new_text_edit(
        id: impl Into<super::Id>,
        position: impl Into<super::Position>, // Center
        width: impl Into<super::Value>,
        rows: u64,
        font_size: f64,
        style: super::style::Bundle,
    ) -> Self {
        Self::TextEdit(TextEdit::new(
            id.into(),
            position.into(),
            width.into(),
            rows,
            font_size,
            style,
        ))
    }
    pub fn new_image(
        id: impl Into<super::Id>,
        position: impl Into<super::Position>, // Center
        size: impl Into<super::Vector>,
        style: super::Style,
        image: crate::assets::sprite::SpriteId,

    ) -> Self {
        Self::Image(image::Image::new(
            id.into(),
            position.into(),
            size.into(),
            style,
            image,
        ))
    }
}

/// Getters
impl Element {
    //Credit: Rust Programming discord: bruh![moment] (170999103482757120)
    // https://discord.com/channels/273534239310479360/1120124565591425034/1162574037633990736
    // Could be done by a macro lmao
    pub fn try_inner<T: TElement>(&self) -> Option<&T> {
        match self {
            Self::Button(inner) => (inner as &dyn std::any::Any).downcast_ref(),
            Self::Graph(inner) => (inner as &dyn std::any::Any).downcast_ref(),
            Self::Text(inner) => (inner as &dyn std::any::Any).downcast_ref(),
            Self::TextEdit(inner) => (inner as &dyn std::any::Any).downcast_ref(),
            Self::Image(inner) => (inner as &dyn std::any::Any).downcast_ref(),
        }
    }
    pub fn inner<T: TElement>(&self) -> &T {
        self.try_inner().expect("Wrong widget type")
    }

    pub fn try_inner_mut<T: TElement>(&mut self) -> Option<&mut T> {
        match self {
            Self::Button(inner) => (inner as &mut dyn std::any::Any).downcast_mut(),
            Self::Graph(inner) => (inner as &mut dyn std::any::Any).downcast_mut(),
            Self::Text(inner) => (inner as &mut dyn std::any::Any).downcast_mut(),
            Self::TextEdit(inner) => (inner as &mut dyn std::any::Any).downcast_mut(),
            Self::Image(inner) => (inner as &mut dyn std::any::Any).downcast_mut(),
        }
    }
    pub fn inner_mut<T: TElement>(&mut self) -> &mut T {
        self.try_inner_mut().expect("Wrong widget type")
    }

    pub fn inner_as_trait(&self) -> &dyn TElement {
        match self {
            Self::Button(inner) => inner,
            Self::Graph(inner) => inner,
            Self::Text(inner) => inner,
            Self::TextEdit(inner) => inner,
            Self::Image(inner) => inner,
        }
    }
    pub fn inner_as_trait_mut(&mut self) -> &mut dyn TElement {
        match self {
            Self::Button(inner) => inner,
            Self::Graph(inner) => inner,
            Self::Text(inner) => inner,
            Self::TextEdit(inner) => inner,
            Self::Image(inner) => inner,
        }
    }

    // this function creates wayyy too much asm bloat
    // pub fn inner_as_trait_boxed(&mut self) -> Box<&mut dyn TElement> {
    //     match self {
    //         Self::Button(inner) => Box::new(inner),
    //         Self::Graph(inner) => Box::new(inner),
    //     }
    // }
}
macro_rules! gen_trait_fn_refmut {
    ($fn_name:ident $(, $arg:ident : $arg_ty:ty)* => $ret_ty:ty) => {
        pub fn $fn_name(&mut self, $($arg : $arg_ty),*) -> $ret_ty {
            self.inner_as_trait_mut().$fn_name($($arg),*)
        }
    };
    ($fn_name:ident => $ret_ty:ty) => {
        pub fn $fn_name(&mut self) -> $ret_ty {
            self.inner_as_trait().$fn_name()
        }
    };
}

macro_rules! gen_trait_fn_ref{
    ($fn_name:ident $(, $arg:ident : $arg_ty:ty)* => $ret_ty:ty) => {
        pub fn $fn_name(&self, $($arg : $arg_ty),*) -> $ret_ty {
            self.inner_as_trait().$fn_name($($arg),*)
        }
    };
    ($fn_name:ident => $ret_ty:ty) => {
        pub fn $fn_name(&self) -> $ret_ty {
            self.inner_as_trait().$fn_name()
        }
    };
}

// macro_rules! gen_trait_fn_value {
//     ($fn_name:ident $(, $arg:ident : $arg_ty:ty)* => $ret_ty:ty) => {
//         pub fn $fn_name(self, $($arg : $arg_ty),*) -> $ret_ty {
//             self.inner_as_trait().$fn_name($($arg),*)
//         }
//     };
//     ($fn_name:ident => $ret_ty:ty) => {
//         pub fn $fn_name(self) -> $ret_ty {
//             self.inner_as_trait().$fn_name()
//         }
//     };
// }

// macro_rules! gen_trait_fn_noself {
//     ($fn_name:ident $(, $arg:ident : $arg_ty:ty)* => $ret_ty:ty) => {
//         pub fn $fn_name($($arg : $arg_ty),*) -> $ret_ty {
//             self.inner_as_trait().$fn_name($($arg),*)
//         }
//     };
//     ($fn_name:ident => $ret_ty:ty) => {
//         pub fn $fn_name() -> $ret_ty {
//             self.inner_as_trait().$fn_name()
//         }
//     };
// }
/// This is so you don't need to import the trait everytime you want to use an Element, you can short circuit it by doing Element::trait_function()
#[allow(dead_code)]
impl Element {
    gen_trait_fn_refmut!(
        draw,
        _ctx: &mut ggez::Context,
        _back: &mut ggez::graphics::MeshBuilder,
        _ui: &mut ggez::graphics::MeshBuilder,
        _front: &mut ggez::graphics::MeshBuilder,
        _render_request: &mut crate::render::RenderRequest
        => ggez::GameResult
    );
    gen_trait_fn_ref!(
        get_size_value
        => &super::Vector
    );

    gen_trait_fn_ref!(
        get_pos_value
        => &super::Position
    );

    gen_trait_fn_ref!(
        get_id
        => super::Id
    );
    /*
        ↑
        Required
        Auto impls
        ↓
    */
    gen_trait_fn_ref!(
        get_computed_size,
        ctx: &mut ggez::Context
        => shared::maths::Vec2
    );
    gen_trait_fn_ref!(
        get_computed_pos,
        ctx: &mut ggez::Context,
        size_opt: Option<shared::maths::Vec2>
        => shared::maths::Point
    );
    gen_trait_fn_ref!(
        get_computed_rect,

        ctx: &mut ggez::Context
        => shared::maths::Rect
    );

    /*
        Events
    */
    gen_trait_fn_refmut!(
        on_mouse_press,
        _button: ggez::input::mouse::MouseButton,
        _position: shared::maths::Point,
        _ctx: &mut ggez::Context
        => ()
    );
    gen_trait_fn_refmut!(

        on_mouse_release,
        _button: ggez::input::mouse::MouseButton,
        _position: shared::maths::Point,
        _ctx: &mut ggez::Context
        => ()
    );
    gen_trait_fn_refmut!(

        on_mouse_motion,
        _position: shared::maths::Point,
        _delta: shared::maths::Point,
        _ctx: &mut ggez::Context
        => ()
    );
    gen_trait_fn_refmut!(
        on_mouse_wheel,
        _delta: shared::maths::Point,
        _ctx: &mut ggez::Context
        => ()
    );
    gen_trait_fn_refmut!(
        on_key_down,
        _key: ggez::input::keyboard::KeyInput,
        _repeated: bool,
        _ctx: &mut ggez::Context
        => ()
    );
    gen_trait_fn_refmut!(
        on_key_up,
        _key: ggez::input::keyboard::KeyInput,
        _ctx: &mut ggez::Context
        => ()
    );
    gen_trait_fn_refmut!(
        on_text_input,
        _character: char,
        _ctx: &mut ggez::Context
        => ()
    );
    gen_trait_fn_refmut!(
        on_new_frame
        =>()
    );
}
