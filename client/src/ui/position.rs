// The position of an element is it's center
#[derive(Clone, Debug)]
pub enum Position {
    Value(super::Vector),
    Anchor {
        anchor: super::Anchor,
        offset: super::Vector,
    },
}

impl Position {
    pub fn new_value(value: impl Into<super::Vector>) -> Self {
        let value = value.into();
        Self::Value(value)
    }

    pub fn new_anchor(anchor: super::Anchor, offset: impl Into<super::Vector>) -> Self {
        Self::Anchor {
            anchor,
            offset: offset.into(),
        }
    }

    pub fn compute(
        &self,
        ctx: &mut ggez::Context,
        element_size: shared::maths::Point,
    ) -> shared::maths::Point {
        match self {
            Position::Value(pt) => {
                shared::maths::Point::new(pt.x().compute(ctx), pt.y().compute(ctx))
            }
            Position::Anchor { anchor, offset } => {
                let offset =
                    shared::maths::Point::new(offset.x().compute(ctx), offset.y().compute(ctx));
                let drawable_size: shared::maths::Point = ctx.gfx.drawable_size().into();

                let anchor_position = anchor.compute(drawable_size, element_size);

                anchor_position + offset
            }
        }
    }
}

impl From<super::Anchor> for Position {
    fn from(value: super::Anchor) -> Self {
        Self::new_anchor(value, (0., 0.))
    }
}

impl<T: Into<super::Vector>> From<T> for Position {
    fn from(value: T) -> Self {
        Self::new_value(value.into())
    }
}

impl<T: Into<super::Vector>> From<(super::Anchor, T)> for Position {
    fn from(value: (super::Anchor, T)) -> Self {
        Self::new_anchor(value.0, value.1)
    }
}

// impl From<(super::Anchor, (f64, f64))> for Position {
//     fn from(value: (super::Anchor, (f64, f64))) -> Self {
//         Self::new_anchor(value.0, value.1)
//     }
// }
