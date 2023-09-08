use super::InnerRect;
use crate::maths;

#[derive(Clone, Copy, Debug)]
pub struct PointCache {
    points: [maths::Point; 4],
}

impl PointCache {
    pub fn new(inner_rect: impl Into<InnerRect>) -> Self {
        Self {
            points: inner_rect.into().r_points(),
        }
    }
    pub fn r_topleft(&self) -> maths::Point {
        *self.points.get(0).unwrap()
    }
    pub fn r_topright(&self) -> maths::Point {
        *self.points.get(1).unwrap()
    }
    pub fn r_botright(&self) -> maths::Point {
        *self.points.get(2).unwrap()
    }
    pub fn r_botleft(&self) -> maths::Point {
        *self.points.get(3).unwrap()
    }
    pub fn r_lines(&self) -> [maths::Line; 4] {
        [
            maths::Line::new(self.r_topleft(), self.r_topright()),
            maths::Line::new(self.r_topright(), self.r_botright()),
            maths::Line::new(self.r_botright(), self.r_botleft()),
            maths::Line::new(self.r_botleft(), self.r_topleft()),
        ]
    }
    pub fn r_points(&self) -> [maths::Point; 4] {
        [
            self.r_topleft(),
            self.r_topright(),
            self.r_botright(),
            self.r_botleft(),
        ]
    }
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
