mod button;

pub struct Element {
    pub t: ElementType,
    pub id: super::Id,
    pub position: ElementPosition,
    pub size: ggez::mint::Point2<super::Value>,
    pub state: State,
}

pub enum ElementType {
    Button(button::Button),
}

pub enum ElementPosition {
    Value(ggez::mint::Point2<super::Value>),
    Anchor {
        anchor: Anchor,
        offset: ggez::mint::Point2<super::Value>,
    },
}

pub enum Anchor {
    CenterCenter,
    Topleft,
    TopCenter,
    TopRight,
    RightCenter,
    BotRight,
    BotCenter,
    BotLeft,
    LeftCenter,
}

#[derive(Default)]
pub struct State {
    pub hovered: bool,
    pub clicked: bool,
    pub focussed: bool,
}

impl ElementType {
    pub fn new_button() -> Self {
        Self::Button(button::Button {})
    }
}

impl ElementPosition {
    pub fn new_value(value: impl Into<(super::Value, super::Value)>) -> Self {
        let value = value.into();
        Self::Value(ggez::mint::Point2::from([value.0, value.1]))
    }
    pub fn new_anchor(
        anchor: Anchor,
        offset: impl Into<Option<ggez::mint::Point2<super::Value>>>,
    ) -> Self {
        let offset = offset.into().unwrap_or_else(|| {
            ggez::mint::Point2::from([super::Value::Fixed(0.), super::Value::Fixed(0.)])
        });
        Self::Anchor { anchor, offset }
    }

    pub fn compute(
        &self,
        ctx: &mut ggez::Context,
        element_size: shared::maths::Point,
    ) -> shared::maths::Point {
        match self {
            ElementPosition::Value(pt) => {
                shared::maths::Point::new(pt.x.compute(ctx), pt.y.compute(ctx))
            }
            ElementPosition::Anchor { anchor, offset } => {
                let offset =
                    shared::maths::Point::new(offset.x.compute(ctx), offset.y.compute(ctx));
                let drawable_size: shared::maths::Point = ctx.gfx.drawable_size().into();

                let anchor_position = anchor.compute(drawable_size, element_size);

                anchor_position + offset
            }
        }
    }
}

impl Anchor {
    /// Returns the topleft point of the element
    pub fn compute(
        &self,
        drawable_size: shared::maths::Point,
        element_size: shared::maths::Point,
    ) -> shared::maths::Point {
        match self {
            Anchor::CenterCenter => {
                shared::maths::Point::new(drawable_size.x / 2., drawable_size.y / 2.)
                    - element_size / 2.
            }
            Anchor::Topleft => shared::maths::Point::ZERO,
            Anchor::TopCenter => {
                shared::maths::Point::new(drawable_size.x / 2. - element_size.x / 2., 0.)
            }
            Anchor::TopRight => shared::maths::Point::new(drawable_size.x - element_size.x, 0.),
            Anchor::RightCenter => shared::maths::Point::new(
                drawable_size.x - element_size.x,
                drawable_size.y / 2. - element_size.y / 2.,
            ),
            Anchor::BotRight => shared::maths::Point::new(
                drawable_size.x - element_size.x,
                drawable_size.y - element_size.y,
            ),
            Anchor::BotCenter => shared::maths::Point::new(
                drawable_size.x / 2. - element_size.x / 2.,
                drawable_size.y - element_size.y,
            ),
            Anchor::BotLeft => shared::maths::Point::new(0., drawable_size.y - element_size.y),
            Anchor::LeftCenter => {
                shared::maths::Point::new(0., drawable_size.y / 2. - element_size.y / 2.)
            }
        }
    }
}

impl Element {
    pub fn new(
        t: ElementType,
        position: ElementPosition,
        size: (impl Into<super::Value>, impl Into<super::Value>),
    ) -> Self {
        let x = size.0.into();
        let y = size.1.into();
        let size = ggez::mint::Point2::from([x, y]);
        Element {
            t,
            id: String::from("This is a test"),
            position,
            size,
            state: State::default(),
        }
    }
    pub fn compute_rect(&self, ctx: &mut ggez::Context) -> ggez::graphics::Rect {
        let size = shared::maths::Point::new(self.size.x.compute(ctx), self.size.y.compute(ctx));

        let position = self.position.compute(ctx, size);

        ggez::graphics::Rect::new(
            position.x as f32,
            position.y as f32,
            size.x as f32,
            size.y as f32,
        )
    }

    pub fn draw(
        &mut self,
        ctx: &mut ggez::Context,
        global_mesh: &mut ggez::graphics::MeshBuilder,
        _render_request: &mut crate::render::RenderRequest
    ) -> ggez::GameResult {
        let rect = self.compute_rect(ctx);
        match &self.t {
            ElementType::Button(_btn) => {
                global_mesh.rectangle(
                    ggez::graphics::DrawMode::fill(),
                    rect,
                    ggez::graphics::Color::from_rgb(0, 175, 150),
                )?;
            }
        }

        Ok(())
    }
}
