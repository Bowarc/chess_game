pub mod element;
pub mod event;
pub mod value;

pub use value::Value;

pub type Id = String;

#[derive(Default)]
pub struct Ui {
    elements: Vec<element::Element>,
    events: std::collections::VecDeque<event::Event>,
}

impl Ui {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_element(&mut self, elem: element::Element) {
        self.elements.push(elem)
    }
    pub fn update(&mut self, ctx: &mut ggez::Context) {
        // If new elements are focussed, we need to unfocuss every other elements
        let mut new_elements_focussed = Vec::<Id>::new();

        while let Some(evnt) = self.events.pop_front() {
            for elem in self.elements.iter_mut() {
                let rect = elem.compute_rect(ctx);
                match evnt {
                    event::Event::MousePress {
                        button: _,
                        position,
                    } => {
                        if rect.contains(position) {
                            elem.state.clicked = true;
                            elem.state.focussed = true;
                        }
                        new_elements_focussed.push(elem.id.clone())
                    }
                    event::Event::MouseRelease {
                        button: _,
                        position,
                    } => {
                        if rect.contains(position) {
                            elem.state.clicked = false;
                        }
                    }
                    event::Event::MouseWheel { pos: _ } => {
                        // Idk how to handle this event for now
                    }
                    event::Event::KeyDown {
                        input: _,
                        repeated: _,
                    } => {
                        // Idk how to handle this event for now
                    }
                    event::Event::KeyUp { input: _ } => {
                        // Idk how to handle this event for now
                    }
                    event::Event::TextInput { character: _ } => {
                        // Idk how to handle this event for now
                    }
                }
            }
        }

        if !new_elements_focussed.is_empty() {
            for elem in self.elements.iter_mut() {
                if new_elements_focussed.contains(&elem.id) {
                    continue;
                }
                elem.state.focussed = false
            }
        }

        self.events.clear()
    }

    pub fn draw(
        &mut self,
        ctx: &mut ggez::Context,
        canvas: &mut ggez::graphics::Canvas,
    ) -> ggez::GameResult {
        let mut global_mesh = ggez::graphics::MeshBuilder::new();
        for elem in self.elements.iter_mut() {
            elem.draw(ctx, canvas, &mut global_mesh)?
        }

        canvas.draw(
            &ggez::graphics::Mesh::from_data(ctx, global_mesh.build()),
            ggez::graphics::DrawParam::new(),
        );
        Ok(())
    }
}

/// Event registration
impl Ui {
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
    pub fn register_mouse_wheel(&mut self, x: f32, y: f32) {
        self.events.push_back(event::Event::MouseWheel {
            pos: shared::maths::Point::new(x as f64, y as f64),
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
