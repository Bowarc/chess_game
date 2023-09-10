#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RenderLog {
    elements: i32,
    sprites: i32,
    meshes: i32,
    texts: i32,
    draw_calls: i32,
    sprites_not_found: i32,
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
        }
    }
    pub fn on_draw_call(&mut self) {
        self.draw_calls += 1;
    }
    pub fn on_sprite(&mut self) {
        self.sprites += 1;
        self.elements += 1;
    }
    pub fn on_mesh(&mut self) {
        self.meshes += 1;
        self.elements += 1;
    }
    pub fn on_text(&mut self) {
        self.texts += 1;
        self.elements += 1;
    }
    pub fn on_sprites_not_found(&mut self) {
        self.sprites_not_found += 1;
    }
    pub fn elements(&self) -> i32 {
        self.elements
    }
    pub fn sprites(&self) -> i32 {
        self.sprites
    }
    pub fn meshes(&self) -> i32 {
        self.meshes
    }
    pub fn texts(&self) -> i32 {
        self.texts
    }
    pub fn draw_calls(&self) -> i32 {
        self.draw_calls
    }
    pub fn sprites_not_found(&self) -> i32 {
        self.sprites_not_found
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
        }
    }
}

impl std::ops::AddAssign for RenderLog {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}
impl std::ops::SubAssign for RenderLog {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
