

#[derive(Clone, Debug)]
pub struct Vector{
    x: super::Value,
    y: super::Value
}

impl Vector {
    pub fn new(x: impl Into<super::Value>, y: impl Into<super::Value>) -> Self{
        Self{
            x: x.into(), y: y.into(),
        }
    }

    pub fn new_value(value: impl Into<(super::Value, super::Value)>) -> Self {
        let value = value.into();
        Self::new(value.0, value.1)
    }
    pub fn new_anchor(
        anchor: super::Anchor,
        offset: (impl Into<super::Value>, impl Into<super::Value>),
    ) -> Self {
        let offsetx = offset.0.into();
        let offsety = offset.1.into();
        let anchor_value = anchor.as_value();
        let anchor_value =(anchor_value.0 + offsetx, anchor_value.1 +offsety);
        Self::new(anchor_value.0, anchor_value.1)
    }

    pub fn compute(
        &self,
        ctx: &mut ggez::Context,
        element_size: shared::maths::Point,
    ) -> shared::maths::Point {
        shared::maths::Point::new(self.x.compute(ctx), self.y.compute(ctx))
    }

    #[inline]
    pub fn x(&self) -> super::Value{
        self.x.clone()
    }
    #[inline]
    pub fn y(&self) -> super::Value{
        self.y.clone()
    }

    pub fn xy(&self) -> (super::Value, super::Value){
        (self.x(), self.y())
    }
}

impl From<(super::Value, super::Value)> for Vector {
    fn from(value: (super::Value, super::Value)) -> Self {
        Self::new_value(value)
    }
}

impl From<super::Anchor> for Vector {
    fn from(value: super::Anchor) -> Self {
        Self::new_anchor(value, (0., 0.))
    }
}

impl From<(super::Anchor, shared::maths::Vec2)> for Vector {
    fn from(value: (super::Anchor, shared::maths::Vec2)) -> Self {
        Self::new_anchor(value.0, (value.1.x, value.1.y))
    }
}

impl std::ops::Add<Vector> for Vector{
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Self::new(self.x+rhs.x, self.y + rhs.y)
    }
}