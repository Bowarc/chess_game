#[derive(Copy, Clone, Debug)]
pub enum Event {
    MousePress {
        button: ggez::input::mouse::MouseButton,
        position: shared::maths::Point,
    },
    MouseRelease {
        button: ggez::input::mouse::MouseButton,
        position: shared::maths::Point,
    },
    MouseMotion {
        position: shared::maths::Point,
        delta: shared::maths::Vec2,
    },
    MouseWheel {
        delta: shared::maths::Point,
    },
    KeyDown {
        key: ggez::input::keyboard::KeyInput,
        repeated: bool,
    },
    KeyUp {
        key: ggez::input::keyboard::KeyInput,
    },
    TextInput {
        character: char,
    },
}
