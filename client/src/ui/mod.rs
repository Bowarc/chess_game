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

pub type Id = String;

#[derive(Default)]
pub struct UiManager {
    elements: Vec<element::Element>,
    events: std::collections::VecDeque<event::Event>,
}

impl UiManager {
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
                        if shared::maths::collision::point_rect(position, rect) {
                            elem.state.clicked = true;
                            elem.state.focussed = true;
                        }
                        new_elements_focussed.push(elem.id.clone())
                    }
                    event::Event::MouseRelease {
                        button: _,
                        position,
                    } => {
                        if shared::maths::collision::point_rect(position, rect) {
                            elem.state.clicked = false;
                        }
                    }
                    event::Event::MouseMotion { pos, delta: _ } => {
                        if shared::maths::collision::point_rect(pos, rect) {
                            elem.state.hovered = true
                        } else {
                            elem.state.hovered = false
                        }
                    }

                    event::Event::MouseWheel { delta: _ } => {
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
            for elem in self
                .elements
                .iter_mut()
                .filter(|el| !new_elements_focussed.contains(&el.id))
            {
                elem.state.clicked = false;
                elem.state.focussed = false
            }
        }

        self.events.clear()
    }

    pub fn draw(
        &mut self,
        ctx: &mut ggez::Context,
        render_request: &mut crate::render::RenderRequest,
    ) -> ggez::GameResult {
        let mut global_mesh = ggez::graphics::MeshBuilder::new();
        for elem in self.elements.iter_mut() {
            elem.draw(ctx, &mut global_mesh, render_request)?
        }

        // canvas.draw(
        //     &ggez::graphics::Mesh::from_data(ctx, global_mesh.build()),
        //     ggez::graphics::DrawParam::new(),
        // );
        render_request.add(
            global_mesh,
            crate::render::DrawParam::default(),
            crate::render::Layer::Ui,
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
