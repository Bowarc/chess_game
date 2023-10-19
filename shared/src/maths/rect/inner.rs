use crate::maths;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InnerRect {
    center: maths::Point,
    size: maths::Vec2,
    rotation: f64,
}

impl InnerRect {
    pub fn new(
        topleft: impl Into<maths::Point>,
        size: impl Into<maths::Vec2>,
        rotation: impl Into<f64>,
    ) -> Self {
        let topleft = topleft.into();
        let size = size.into();
        let rotation = rotation.into();

        Self {
            center: topleft + size * 0.5,
            size,
            rotation,
        }
    }

    pub fn center(&self) -> maths::Point {
        self.center
    }
    pub fn set_center(&mut self, new_center: impl Into<maths::Point>) {
        self.center = new_center.into()
    }
    pub fn size(&self) -> maths::Vec2 {
        self.size
    }
    pub fn set_size(&mut self, new_size: impl Into<maths::Vec2>) {
        self.size = new_size.into()
    }
    pub fn rotation(&self) -> f64 {
        self.rotation
    }
    pub fn set_rotation(&mut self, new_rotation: impl Into<f64>) {
        self.rotation = new_rotation.into()
    }
    /// Axis aligned top left point
    pub fn aa_topleft(&self) -> maths::Point {
        maths::Point::new(
            self.center.x - self.size.x * 0.5,
            self.center.y - self.size.y * 0.5,
        )
    }
    /// Axis aligned top right point
    pub fn aa_topright(&self) -> maths::Point {
        maths::Point::new(
            self.center.x + self.size.x * 0.5,
            self.center.y - self.size.y * 0.5,
        )
    }
    /// Axis aligned bot right point
    pub fn aa_botright(&self) -> maths::Point {
        maths::Point::new(
            self.center.x + self.size.x * 0.5,
            self.center.y + self.size.y * 0.5,
        )
    }
    /// Axis aligned bot left point
    pub fn aa_botleft(&self) -> maths::Point {
        maths::Point::new(
            self.center.x - self.size.x * 0.5,
            self.center.y + self.size.y * 0.5,
        )
    }
    /// Axis aligned lines
    pub fn aa_lines(&self) -> [maths::Line; 4] {
        [
            maths::Line::new(self.aa_topleft(), self.aa_topright()),
            maths::Line::new(self.aa_topright(), self.aa_botright()),
            maths::Line::new(self.aa_botright(), self.aa_botleft()),
            maths::Line::new(self.aa_botleft(), self.aa_topleft()),
        ]
    }
    /// Axis aligned points
    pub fn aa_points(&self) -> [maths::Point; 4] {
        [
            self.aa_topleft(),
            self.aa_topright(),
            self.aa_botright(),
            self.aa_botleft(),
        ]
    }
    /// Axis aligned 5 points
    pub fn aa_points5(&self) -> [maths::Point; 5] {
        [
            self.aa_topleft(),
            self.aa_topright(),
            self.aa_botright(),
            self.aa_botleft(),
            self.aa_topleft(),
        ]
    }
    /// Rotated top left
    pub fn r_topleft(&self) -> maths::Point {
        maths::Point::new_rotated(self.center, self.aa_topleft(), self.rotation)
    }
    /// Rotated top right
    pub fn r_topright(&self) -> maths::Point {
        maths::Point::new_rotated(self.center, self.aa_topright(), self.rotation)
    }
    /// Rotated bot right
    pub fn r_botright(&self) -> maths::Point {
        maths::Point::new_rotated(self.center, self.aa_botright(), self.rotation)
    }
    /// Rotated bot left
    pub fn r_botleft(&self) -> maths::Point {
        maths::Point::new_rotated(self.center, self.aa_botleft(), self.rotation)
    }
    /// Rotated lines
    pub fn r_lines(&self) -> [maths::Line; 4] {
        [
            maths::Line::new(self.r_topleft(), self.r_topright()),
            maths::Line::new(self.r_topright(), self.r_botright()),
            maths::Line::new(self.r_botright(), self.r_botleft()),
            maths::Line::new(self.r_botleft(), self.r_topleft()),
        ]
    }
    /// Rotated points
    pub fn r_points(&self) -> [maths::Point; 4] {
        [
            self.r_topleft(),
            self.r_topright(),
            self.r_botright(),
            self.r_botleft(),
        ]
    }
    /// Rotated points 5
    pub fn r_points5(&self) -> [maths::Point; 5] {
        [
            self.r_topleft(),
            self.r_topright(),
            self.r_botright(),
            self.r_botleft(),
            self.r_topleft(),
        ]
    }
}
