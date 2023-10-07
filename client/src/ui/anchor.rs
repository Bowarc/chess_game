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
