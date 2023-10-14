use crate::ui;

#[derive(Clone, Debug)]
pub enum Position {
    Value(ggez::mint::Point2<ui::Value>),
    Anchor {
        anchor: super::Anchor,
        offset: ggez::mint::Point2<ui::Value>,
    },
}

impl Position {
    pub fn new_value(value: impl Into<(ui::Value, ui::Value)>) -> Self {
        let value = value.into();
        Self::Value(ggez::mint::Point2::from([value.0, value.1]))
    }
    pub fn new_anchor(
        anchor: super::Anchor,

        offset: (impl Into<crate::ui::Value>, impl Into<crate::ui::Value>),
    ) -> Self {
        // let offset = offset.unwrap_or((0., 0.));

        Self::Anchor {
            anchor,
            offset: ggez::mint::Point2::from([offset.0.into(), offset.1.into()]),
        }
    }

    pub fn compute(
        &self,
        ctx: &mut ggez::Context,
        element_size: shared::maths::Point,
    ) -> shared::maths::Point {
        match self {
            Position::Value(pt) => shared::maths::Point::new(pt.x.compute(ctx), pt.y.compute(ctx)),
            Position::Anchor { anchor, offset } => {
                let offset =
                    shared::maths::Point::new(offset.x.compute(ctx), offset.y.compute(ctx));
                let drawable_size: shared::maths::Point = ctx.gfx.drawable_size().into();

                let anchor_position = anchor.compute(drawable_size, element_size);

                anchor_position + offset
            }
        }
    }
}
