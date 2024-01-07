#[derive(Clone, Debug)]
pub struct Vector {
    x: super::Value,
    y: super::Value,
}

impl Vector {
    pub fn new(x: impl Into<super::Value>, y: impl Into<super::Value>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }

    #[inline]
    pub fn x(&self) -> super::Value {
        self.x.clone()
    }
    #[inline]
    pub fn y(&self) -> super::Value {
        self.y.clone()
    }

    pub fn xy(&self) -> (super::Value, super::Value) {
        (self.x(), self.y())
    }

    #[inline]
    pub fn w(&self) -> super::Value {
        self.x()
    }
    #[inline]
    pub fn h(&self) -> super::Value {
        self.y()
    }

    pub fn wh(&self) -> (super::Value, super::Value) {
        self.xy()
    }
}

// impl From<(super::Value, super::Value)> for Vector {
//     fn from(value: (super::Value, super::Value)) -> Self {
//         Self::new_value(value)
//     }
// }

impl<T: Into<super::Value>> From<(T, T)> for Vector {
    fn from(val: (T, T)) -> Self {
        Self::new(val.0.into(), val.1.into())
    }
}

// impl From<super::Anchor> for Vector {
//     fn from(value: super::Anchor) -> Self {
//         Self::new_anchor(value, (0., 0.))
//     }
// }

// impl From<(super::Anchor, shared::maths::Vec2)> for Vector {
//     fn from(value: (super::Anchor, shared::maths::Vec2)) -> Self {
//         Self::new_anchor(value.0, (value.1.x, value.1.y))
//     }
// }

impl<T: Into<Vector>> std::ops::Add<T> for Vector {
    type Output = Vector;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Add<f64> for Vector {
    type Output = Vector;

    fn add(self, rhs: f64) -> Self::Output {
        Self::new(self.x + rhs, self.y + rhs)
    }
}

impl<T: Into<Vector>> std::ops::Sub<T> for Vector {
    type Output = Vector;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Sub<f64> for Vector {
    type Output = Vector;

    fn sub(self, rhs: f64) -> Self::Output {
        Self::new(self.x - rhs, self.y - rhs)
    }
}

impl<T: Into<Vector>> std::ops::Mul<T> for Vector {
    type Output = Vector;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl std::ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl<T: Into<Vector>> std::ops::Div<T> for Vector {
    type Output = Vector;

    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl std::ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}
