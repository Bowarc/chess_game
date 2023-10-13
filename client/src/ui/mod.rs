mod anchor;
pub mod element;
pub mod event;
mod position;
pub mod register;
mod state;
pub mod style;
pub mod value;

pub use anchor::Anchor;
pub use position::Position;
pub use state::State;
pub use style::Style;
pub use value::Value;

pub type Id = shared::id::Id;

#[derive(Default)]
pub struct UiManager {
    elements: Vec<element::Element>,
    events: std::collections::VecDeque<event::Event>,
}

impl UiManager {
    pub fn add_element(&mut self, elem: element::Element) {
        self.elements.push(elem)
    }

    pub fn update(&mut self, ctx: &mut ggez::Context) {
        use element::TElement as _;

        // Re-initializes the new frame part of the element state struct
        self.elements
            .iter_mut()
            .for_each(|el| el.get_state_mut().new_frame());

        while let Some(ev) = self.events.pop_front() {
            for el in self.elements.iter_mut() {
                match ev {
                    event::Event::MousePress { button, position } => {
                        el.on_mouse_press(button, position, ctx)
                    }
                    event::Event::MouseRelease { button, position } => {
                        el.on_mouse_release(button, position, ctx)
                    }
                    event::Event::MouseMotion { pos, delta } => el.on_mouse_motion(pos, delta, ctx),
                    event::Event::MouseWheel { delta } => el.on_mouse_wheel(delta, ctx),
                    event::Event::KeyDown { input, repeated } => {
                        el.on_key_down(input, repeated, ctx)
                    }
                    event::Event::KeyUp { input } => el.on_key_up(input, ctx),
                    event::Event::TextInput { character } => el.on_text_input(character, ctx),
                }
            }
        }
    }

    pub fn draw(
        &mut self,
        ctx: &mut ggez::Context,
        render_request: &mut crate::render::RenderRequest,
    ) -> ggez::GameResult {
        use element::TElement as _;
        let mut background_mesh = ggez::graphics::MeshBuilder::new();
        let mut ui_mesh = ggez::graphics::MeshBuilder::new();
        let mut foreground_mesh = ggez::graphics::MeshBuilder::new();

        for elem in self.elements.iter_mut() {
            elem.draw(
                ctx,
                &mut background_mesh,
                &mut ui_mesh,
                &mut foreground_mesh,
                render_request,
            )?
        }

        render_request.add(
            background_mesh,
            Default::default(),
            crate::render::Layer::UiBackground,
        );

        render_request.add(ui_mesh, Default::default(), crate::render::Layer::Ui);

        render_request.add(
            foreground_mesh,
            Default::default(),
            crate::render::Layer::UiForeground,
        );

        Ok(())
    }
}

/// Event registration
impl UiManager {
    pub fn register_mouse_press(
        &mut self,
        button: ggez::input::mouse::MouseButton,
        x: f32,
        y: f32,
    ) {
        self.events.push_back(event::Event::MousePress {
            button,
            position: shared::maths::Point::new(x as f64, y as f64),
        })
    }
    pub fn register_mouse_release(
        &mut self,
        button: ggez::input::mouse::MouseButton,
        x: f32,
        y: f32,
    ) {
        self.events.push_back(event::Event::MouseRelease {
            button,
            position: shared::maths::Point::new(x as f64, y as f64),
        })
    }
    pub fn register_mouse_motion(&mut self, x: f32, y: f32, dx: f32, dy: f32) {
        self.events.push_back(event::Event::MouseMotion {
            pos: shared::maths::Point::new(x as f64, y as f64),
            delta: shared::maths::Vec2::new(dx as f64, dy as f64),
        });
    }
    pub fn register_mouse_wheel(&mut self, x: f32, y: f32) {
        self.events.push_back(event::Event::MouseWheel {
            delta: shared::maths::Point::new(x as f64, y as f64),
        })
    }
    pub fn register_key_down(&mut self, input: ggez::input::keyboard::KeyInput, repeated: bool) {
        self.events
            .push_back(event::Event::KeyDown { input, repeated })
    }
    pub fn register_key_up(&mut self, input: ggez::input::keyboard::KeyInput) {
        self.events.push_back(event::Event::KeyUp { input })
    }
    pub fn register_text_input(&mut self, character: char) {
        self.events.push_back(event::Event::TextInput { character })
    }
}

/// Getters
impl UiManager {
    pub fn get_element(&mut self, id: Id) -> Option<&element::Element> {
        use element::TElement as _;

        if let Some(index) = self
            .elements
            .iter()
            .enumerate()
            .flat_map(|(i, el)| if el.get_id() == &id { Some(i) } else { None })
            .collect::<Vec<usize>>()
            .first()
        {
            Some(self.elements.get(*index).unwrap())
        } else {
            None
        }
    }
}
