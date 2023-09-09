#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RenderLog {
    pub elements: i32,
    pub sprites: i32,
    pub meshes: i32,
    pub texts: i32,
    pub draw_calls: i32,
    pub sprites_not_found: i32,
    pub ordering_sort_duration: std::time::Duration,
}
impl RenderLog {
    pub fn new() -> Self {
        Self {
            elements: 0,
            sprites: 0,
            meshes: 0,
            texts: 0,
            draw_calls: 0,

            sprites_not_found: 0,
            ordering_sort_duration: std::time::Duration::ZERO,
        }
    }
}

impl std::ops::Add<RenderLog> for RenderLog {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            elements: self.elements + other.elements,
            sprites: self.sprites + other.sprites,
            meshes: self.meshes + other.meshes,
            texts: self.texts + other.texts,
            draw_calls: self.draw_calls + other.draw_calls,
            sprites_not_found: self.sprites_not_found + other.sprites_not_found,
            ordering_sort_duration: self.ordering_sort_duration + other.ordering_sort_duration,
        }
    }
}

impl std::ops::Sub<RenderLog> for RenderLog {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            elements: self.elements - other.elements,
            sprites: self.sprites - other.sprites,
            meshes: self.meshes - other.meshes,
            texts: self.texts - other.texts,
            draw_calls: self.draw_calls - other.draw_calls,
            sprites_not_found: self.sprites_not_found - other.sprites_not_found,
            ordering_sort_duration: self.ordering_sort_duration - other.ordering_sort_duration,
        }
    }
}

impl std::ops::AddAssign for RenderLog {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            elements: self.elements + other.elements,
            sprites: self.sprites + other.sprites,
            meshes: self.meshes + other.meshes,
            texts: self.texts + other.texts,
            draw_calls: self.draw_calls + other.draw_calls,
            sprites_not_found: self.sprites_not_found + other.sprites_not_found,
            ordering_sort_duration: self.ordering_sort_duration + other.ordering_sort_duration,
        };
    }
}
impl std::ops::SubAssign for RenderLog {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            elements: self.elements - other.elements,
            sprites: self.sprites - other.sprites,
            meshes: self.meshes - other.meshes,
            texts: self.texts - other.texts,
            draw_calls: self.draw_calls - other.draw_calls,
            sprites_not_found: self.sprites_not_found - other.sprites_not_found,
            ordering_sort_duration: self.ordering_sort_duration - other.ordering_sort_duration,
        };
    }
}
