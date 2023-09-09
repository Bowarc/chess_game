mod font_id;
pub use font_id::FontId;

pub struct FontBank {}

impl FontBank {
    pub fn new() -> Self {
        Self {}
    }
}

// impl super::Bank<FontId, ?> for FontBank {
//     fn update(&mut self) {
//         todo!()
//     }
//     fn get(&mut self, _: FontId, _: super::loader::Handle) -> &mut ? {
//         todo!()
//     }
// }
