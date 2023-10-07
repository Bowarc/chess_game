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
        pos: shared::maths::Point,
        delta: shared::maths::Vec2,
    },
    MouseWheel {
        delta: shared::maths::Point, // not sure of what to call it
    },
    KeyDown {
        input: ggez::input::keyboard::KeyInput,
        repeated: bool,
    },
    KeyUp {
        input: ggez::input::keyboard::KeyInput,
    },
    TextInput {
        character: char,
    },
}
