/*
Used by any1 that need to make calls to the loader thread
*/

use crate::assets::{font, sound, sprite};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Request {
    Font(font::FontId),
    Sound(sound::SoundId),
    Sprite(sprite::SpriteId),
}

impl Request {
    pub fn sprite_id(&self) -> sprite::SpriteId {
        match self {
            Request::Sprite(id) => *id,
            _ => panic!("You've done fucked up"),
        }
    }
    pub fn sound_id(&self) -> sound::SoundId {
        match self {
            Request::Sound(id) => *id,
            _ => panic!("You've done fucked up"),
        }
    }
    pub fn font_id(&self) -> font::FontId {
        match self {
            Request::Font(id) => *id,
            _ => panic!("You've done fucked up"),
        }
    }
}
