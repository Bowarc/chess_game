// pub const DEFAULT_TEXT_VALUE_TOKEN: &str = "$"; // Relic of a bygone past

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Fixed(f64),
    Magic(MagicValue),
    Mutiple(Box<Value>, ValueOperation, Box<Value>),
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ValueOperation {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MagicValue {
    ScreenSizeW,
    ScreenSizeH,
    MousePosX,
    MousePosY,
}
impl Value {
    pub fn compute(&self, ctx: &mut ggez::Context) -> f64 {
        match self {
            Value::Fixed(v) => *v,
            Value::Magic(v) => v.compute(ctx),
            Value::Mutiple(v1, op, v2) => match op {
                ValueOperation::Add => v1.compute(ctx) + v2.compute(ctx),
                ValueOperation::Sub => v1.compute(ctx) - v2.compute(ctx),
                ValueOperation::Mul => v1.compute(ctx) * v2.compute(ctx),
                ValueOperation::Div => v1.compute(ctx) / v2.compute(ctx),
            },
        }
    }

    pub fn fixed(nbr: f64) -> Self {
        Self::Fixed(nbr)
    }

    pub fn magic(magic: MagicValue) -> Self {
        Self::Magic(magic)
    }
    pub fn multiple(v1: Self, op: ValueOperation, v2: Self) -> Self {
        Self::Mutiple(Box::new(v1), op, Box::new(v2))
    }
}

impl MagicValue {
    pub fn compute(&self, ctx: &ggez::Context) -> f64 {
        match self {
            MagicValue::ScreenSizeW => ctx.gfx.drawable_size().0 as f64,
            MagicValue::ScreenSizeH => ctx.gfx.drawable_size().1 as f64,
            MagicValue::MousePosX => ctx.mouse.position().x as f64,
            MagicValue::MousePosY => ctx.mouse.position().y as f64,
        }
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Fixed(value)
    }
}

impl From<MagicValue> for Value {
    fn from(value: MagicValue) -> Self {
        Value::Magic(value)
    }
}

impl From<(Value, ValueOperation, Value)> for Value {
    fn from(value: (Value, ValueOperation, Value)) -> Self {
        Value::Mutiple(Box::new(value.0), value.1, Box::new(value.2))
    }
}

///////////////////
//      ADD      //
///////////////////

// This allows:
// Value::Fixed(1.) + 10.;
// Value::Fixed(1.) + MagicValue::ScreenSizeW;
// Value::Fixed(1.) + Value::Fixed(1.);
impl<T: Into<Value>> std::ops::Add<T> for Value {
    type Output = Value;
    fn add(self, other: T) -> Self::Output {
        Value::Mutiple(Box::new(self), ValueOperation::Add, Box::new(other.into()))
    }
}

// This allows:
// MagicValue::ScreenSizeW + 10.;
// MagicValue::ScreenSizeW + MagicValue::ScreenSizeW;
// MagicValue::ScreenSizeW + Value::Fixed(1.);
impl<T: Into<Value>> std::ops::Add<T> for MagicValue {
    type Output = Value;
    fn add(self, other: T) -> Self::Output {
        Value::Mutiple(
            Box::new(self.into()),
            ValueOperation::Add,
            Box::new(other.into()),
        )
    }
}
// This allows:
// 10. + MagicValue::ScreenSizeW;
impl std::ops::Add<MagicValue> for f64 {
    type Output = Value;
    fn add(self, other: MagicValue) -> Self::Output {
        Value::Mutiple(
            Box::new(self.into()),
            ValueOperation::Add,
            Box::new(other.into()),
        )
    }
}
// This allows:
// 10. + Value::Fixed(1.);
impl std::ops::Add<Value> for f64 {
    type Output = Value;
    fn add(self, other: Value) -> Self::Output {
        Value::Mutiple(Box::new(self.into()), ValueOperation::Add, Box::new(other))
    }
}

///////////////////
//      SUB      //
///////////////////

// This allows:
// Value::Fixed(1.) - 10.;
// Value::Fixed(1.) - MagicValue::ScreenSizeW;
// Value::Fixed(1.) - Value::Fixed(1.);
impl<T: Into<Value>> std::ops::Sub<T> for Value {
    type Output = Value;
    fn sub(self, other: T) -> Self::Output {
        Value::Mutiple(Box::new(self), ValueOperation::Sub, Box::new(other.into()))
    }
}

// This allows:
// MagicValue::ScreenSizeW - 10.;
// MagicValue::ScreenSizeW - MagicValue::ScreenSizeW;
// MagicValue::ScreenSizeW - Value::Fixed(1.);
impl<T: Into<Value>> std::ops::Sub<T> for MagicValue {
    type Output = Value;
    fn sub(self, other: T) -> Self::Output {
        Value::Mutiple(
            Box::new(self.into()),
            ValueOperation::Sub,
            Box::new(other.into()),
        )
    }
}
// This allows:
// 10. - MagicValue::ScreenSizeW;
impl std::ops::Sub<MagicValue> for f64 {
    type Output = Value;
    fn sub(self, other: MagicValue) -> Self::Output {
        Value::Mutiple(
            Box::new(self.into()),
            ValueOperation::Sub,
            Box::new(other.into()),
        )
    }
}
// This allows:
// 10. - Value::Fixed(1.);
impl std::ops::Sub<Value> for f64 {
    type Output = Value;
    fn sub(self, other: Value) -> Self::Output {
        Value::Mutiple(Box::new(self.into()), ValueOperation::Sub, Box::new(other))
    }
}

///////////////////
//      MUL      //
///////////////////

// This allows:
// Value::Fixed(1.) * 10.;
// Value::Fixed(1.) * MagicValue::ScreenSizeW;
// Value::Fixed(1.) * Value::Fixed(1.);
impl<T: Into<Value>> std::ops::Mul<T> for Value {
    type Output = Value;
    fn mul(self, other: T) -> Self::Output {
        Value::Mutiple(Box::new(self), ValueOperation::Mul, Box::new(other.into()))
    }
}

// This allows:
// MagicValue::ScreenSizeW * 10.;
// MagicValue::ScreenSizeW * MagicValue::ScreenSizeW;
// MagicValue::ScreenSizeW * Value::Fixed(1.);
impl<T: Into<Value>> std::ops::Mul<T> for MagicValue {
    type Output = Value;
    fn mul(self, other: T) -> Self::Output {
        Value::Mutiple(
            Box::new(self.into()),
            ValueOperation::Mul,
            Box::new(other.into()),
        )
    }
}
// This allows:
// 10. * MagicValue::ScreenSizeW;
impl std::ops::Mul<MagicValue> for f64 {
    type Output = Value;
    fn mul(self, other: MagicValue) -> Self::Output {
        Value::Mutiple(
            Box::new(self.into()),
            ValueOperation::Mul,
            Box::new(other.into()),
        )
    }
}
// This allows:
// 10. * Value::Fixed(1.);
impl std::ops::Mul<Value> for f64 {
    type Output = Value;
    fn mul(self, other: Value) -> Self::Output {
        Value::Mutiple(Box::new(self.into()), ValueOperation::Mul, Box::new(other))
    }
}

///////////////////
//      DIV      //
///////////////////

// This allows:
// Value::Fixed(1.) / 10.;
// Value::Fixed(1.) / MagicValue::ScreenSizeW;
// Value::Fixed(1.) / Value::Fixed(1.);
impl<T: Into<Value>> std::ops::Div<T> for Value {
    type Output = Value;
    fn div(self, other: T) -> Self::Output {
        Value::Mutiple(Box::new(self), ValueOperation::Div, Box::new(other.into()))
    }
}

// This allows:
// MagicValue::ScreenSizeW / 10.;
// MagicValue::ScreenSizeW / MagicValue::ScreenSizeW;
// MagicValue::ScreenSizeW / Value::Fixed(1.);
impl<T: Into<Value>> std::ops::Div<T> for MagicValue {
    type Output = Value;
    fn div(self, other: T) -> Self::Output {
        Value::Mutiple(
            Box::new(self.into()),
            ValueOperation::Div,
            Box::new(other.into()),
        )
    }
}
// This allows:
// 10. / MagicValue::ScreenSizeW;
impl std::ops::Div<MagicValue> for f64 {
    type Output = Value;
    fn div(self, other: MagicValue) -> Self::Output {
        Value::Mutiple(
            Box::new(self.into()),
            ValueOperation::Div,
            Box::new(other.into()),
        )
    }
}
// This allows:
// 10. / Value::Fixed(1.);
impl std::ops::Div<Value> for f64 {
    type Output = Value;
    fn div(self, other: Value) -> Self::Output {
        Value::Mutiple(Box::new(self.into()), ValueOperation::Div, Box::new(other))
    }
}

#[cfg(test)]
mod operations {
    use super::*;

    #[test]
    fn add() {
        // MagicValue impl Into<Value>
        // f64 impl Into<Value>

        // impl<T: Into<Value>> std::ops::Add<T> for Value
        assert_eq!(
            Value::Fixed(1.) + 10.,
            Value::multiple(Value::fixed(1.), ValueOperation::Add, Value::fixed(10.))
        );
        assert_eq!(
            Value::Fixed(1.) + MagicValue::ScreenSizeW,
            Value::multiple(
                Value::fixed(1.),
                ValueOperation::Add,
                Value::magic(MagicValue::ScreenSizeW)
            )
        );
        assert_eq!(
            Value::Fixed(1.) + Value::Fixed(1.),
            Value::multiple(Value::fixed(1.), ValueOperation::Add, Value::fixed(1.))
        );

        // impl<T: Into<Value>> std::ops::Add<T> for MagicValue {
        assert_eq!(
            MagicValue::ScreenSizeW + 10.,
            Value::multiple(
                Value::magic(MagicValue::ScreenSizeW),
                ValueOperation::Add,
                Value::fixed(10.)
            )
        );
        assert_eq!(
            MagicValue::ScreenSizeW + MagicValue::ScreenSizeW,
            Value::multiple(
                Value::magic(MagicValue::ScreenSizeW),
                ValueOperation::Add,
                Value::magic(MagicValue::ScreenSizeW)
            )
        );
        assert_eq!(
            MagicValue::ScreenSizeW + Value::Fixed(1.),
            Value::multiple(
                Value::magic(MagicValue::ScreenSizeW),
                ValueOperation::Add,
                Value::fixed(1.)
            )
        );

        // impl std::ops::Add<MagicValue> for f64 {
        assert_eq!(
            10. + MagicValue::ScreenSizeW,
            Value::multiple(
                Value::fixed(10.),
                ValueOperation::Add,
                Value::magic(MagicValue::ScreenSizeW),
            )
        );

        // impl std::ops::Add<Value> for f64 {
        assert_eq!(
            10. + Value::Fixed(1.),
            Value::multiple(Value::fixed(10.), ValueOperation::Add, Value::fixed(1.),)
        );
    }

    #[test]
    fn sub() {
        // MagicValue impl Into<Value>
        // f64 impl Into<Value>

        // impl<T: Into<Value>> std::ops::Sub<T> for Value
        assert_eq!(
            Value::Fixed(1.) - 10.,
            Value::multiple(Value::fixed(1.), ValueOperation::Sub, Value::fixed(10.))
        );
        assert_eq!(
            Value::Fixed(1.) - MagicValue::ScreenSizeW,
            Value::multiple(
                Value::fixed(1.),
                ValueOperation::Sub,
                Value::magic(MagicValue::ScreenSizeW)
            )
        );
        assert_eq!(
            Value::Fixed(1.) - Value::Fixed(1.),
            Value::multiple(Value::fixed(1.), ValueOperation::Sub, Value::fixed(1.))
        );

        // impl<T: Into<Value>> std::ops::Sub<T> for MagicValue {
        assert_eq!(
            MagicValue::ScreenSizeW - 10.,
            Value::multiple(
                Value::magic(MagicValue::ScreenSizeW),
                ValueOperation::Sub,
                Value::fixed(10.)
            )
        );
        assert_eq!(
            MagicValue::ScreenSizeW - MagicValue::ScreenSizeW,
            Value::multiple(
                Value::magic(MagicValue::ScreenSizeW),
                ValueOperation::Sub,
                Value::magic(MagicValue::ScreenSizeW)
            )
        );
        assert_eq!(
            MagicValue::ScreenSizeW - Value::Fixed(1.),
            Value::multiple(
                Value::magic(MagicValue::ScreenSizeW),
                ValueOperation::Sub,
                Value::fixed(1.)
            )
        );

        // impl std::ops::Sub<MagicValue> for f64 {
        assert_eq!(
            10. - MagicValue::ScreenSizeW,
            Value::multiple(
                Value::fixed(10.),
                ValueOperation::Sub,
                Value::magic(MagicValue::ScreenSizeW),
            )
        );

        // impl std::ops::Sub<Value> for f64 {
        assert_eq!(
            10. - Value::Fixed(1.),
            Value::multiple(Value::fixed(10.), ValueOperation::Sub, Value::fixed(1.),)
        );
    }
    #[test]
    fn mul() {
        // MagicValue impl Into<Value>
        // f64 impl Into<Value>

        // impl<T: Into<Value>> std::ops::Mul<T> for Value
        assert_eq!(
            Value::Fixed(1.) * 10.,
            Value::multiple(Value::fixed(1.), ValueOperation::Mul, Value::fixed(10.))
        );
        assert_eq!(
            Value::Fixed(1.) * MagicValue::ScreenSizeW,
            Value::multiple(
                Value::fixed(1.),
                ValueOperation::Mul,
                Value::magic(MagicValue::ScreenSizeW)
            )
        );
        assert_eq!(
            Value::Fixed(1.) * Value::Fixed(1.),
            Value::multiple(Value::fixed(1.), ValueOperation::Mul, Value::fixed(1.))
        );

        // impl<T: Into<Value>> std::ops::Mul<T> for MagicValue {
        assert_eq!(
            MagicValue::ScreenSizeW * 10.,
            Value::multiple(
                Value::magic(MagicValue::ScreenSizeW),
                ValueOperation::Mul,
                Value::fixed(10.)
            )
        );
        assert_eq!(
            MagicValue::ScreenSizeW * MagicValue::ScreenSizeW,
            Value::multiple(
                Value::magic(MagicValue::ScreenSizeW),
                ValueOperation::Mul,
                Value::magic(MagicValue::ScreenSizeW)
            )
        );
        assert_eq!(
            MagicValue::ScreenSizeW * Value::Fixed(1.),
            Value::multiple(
                Value::magic(MagicValue::ScreenSizeW),
                ValueOperation::Mul,
                Value::fixed(1.)
            )
        );

        // impl std::ops::Mul<MagicValue> for f64 {
        assert_eq!(
            10. * MagicValue::ScreenSizeW,
            Value::multiple(
                Value::fixed(10.),
                ValueOperation::Mul,
                Value::magic(MagicValue::ScreenSizeW),
            )
        );

        // impl std::ops::Mul<Value> for f64 {
        assert_eq!(
            10. * Value::Fixed(1.),
            Value::multiple(Value::fixed(10.), ValueOperation::Mul, Value::fixed(1.),)
        );
    }
    #[test]
    fn div() {
        // MagicValue impl Into<Value>
        // f64 impl Into<Value>

        // impl<T: Into<Value>> std::ops::Div<T> for Value
        assert_eq!(
            Value::Fixed(1.) / 10.,
            Value::multiple(Value::fixed(1.), ValueOperation::Div, Value::fixed(10.))
        );
        assert_eq!(
            Value::Fixed(1.) / MagicValue::ScreenSizeW,
            Value::multiple(
                Value::fixed(1.),
                ValueOperation::Div,
                Value::magic(MagicValue::ScreenSizeW)
            )
        );
        assert_eq!(
            Value::Fixed(1.) / Value::Fixed(1.),
            Value::multiple(Value::fixed(1.), ValueOperation::Div, Value::fixed(1.))
        );

        // impl<T: Into<Value>> std::ops::Div<T> for MagicValue {
        assert_eq!(
            MagicValue::ScreenSizeW / 10.,
            Value::multiple(
                Value::magic(MagicValue::ScreenSizeW),
                ValueOperation::Div,
                Value::fixed(10.)
            )
        );
        assert_eq!(
            MagicValue::ScreenSizeW / MagicValue::ScreenSizeW,
            Value::multiple(
                Value::magic(MagicValue::ScreenSizeW),
                ValueOperation::Div,
                Value::magic(MagicValue::ScreenSizeW)
            )
        );
        assert_eq!(
            MagicValue::ScreenSizeW / Value::Fixed(1.),
            Value::multiple(
                Value::magic(MagicValue::ScreenSizeW),
                ValueOperation::Div,
                Value::fixed(1.)
            )
        );

        // impl std::ops::Div<MagicValue> for f64 {
        assert_eq!(
            10. / MagicValue::ScreenSizeW,
            Value::multiple(
                Value::fixed(10.),
                ValueOperation::Div,
                Value::magic(MagicValue::ScreenSizeW),
            )
        );

        // impl std::ops::Div<Value> for f64 {
        assert_eq!(
            10. / Value::Fixed(1.),
            Value::multiple(Value::fixed(10.), ValueOperation::Div, Value::fixed(1.),)
        );
    }

    #[test]
    fn operation_order() {
        // The purpose of this one is to check that the Order of operation is kept

        // The 100. is converted into a value else he whole operation creates a f64 instead of a Value.
        // This is for now the biggest flaw of this system
        assert_eq!(
            10. + 20. * Value::fixed(100.),
            Value::multiple(
                Value::fixed(10.),
                ValueOperation::Add,
                Value::multiple(Value::fixed(20.), ValueOperation::Mul, Value::fixed(100.))
            )
        );

        // I also tested that:
        // assert_eq!(2010., (10. + 20. * ggui::Value::fixed(100.)).compute(ctx));
        // doens't create any panic
    }
}
