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

// pub type Id = shared::id::Id;
pub type Id = String;

#[derive(Default)]
pub struct UiManager {
    elements: Vec<element::Element>,
    events: Vec<event::Event>,
}

impl UiManager {
    pub fn add_element(&mut self, elem: element::Element) -> Id {
        let id = elem.get_id();
        assert!(
            self.try_get_element(id.clone()).is_none(),
            "Ui element id collision"
        );
        self.elements.push(elem);
        id
    }

    pub fn update(&mut self, ctx: &mut ggez::Context) {
        // Re-initializes the new frame part of the element state struct
        self.elements.iter_mut().for_each(|el| el.on_new_frame());

        self.events.iter().for_each(|ev| {
            self.elements.iter_mut().for_each(|el| match *ev {
                event::Event::MousePress { button, position } => {
                    el.on_mouse_press(button, position, ctx)
                }
                event::Event::MouseRelease { button, position } => {
                    el.on_mouse_release(button, position, ctx)
                }
                event::Event::MouseMotion { position, delta } => {
                    el.on_mouse_motion(position, delta, ctx)
                }
                event::Event::MouseWheel { delta } => el.on_mouse_wheel(delta, ctx),
                event::Event::KeyDown { key, repeated } => el.on_key_down(key, repeated, ctx),
                event::Event::KeyUp { key } => el.on_key_up(key, ctx),
                event::Event::TextInput { character } => el.on_text_input(character, ctx),
            })
        });
        self.events.clear()
    }

    pub fn draw(
        &mut self,
        ctx: &mut ggez::Context,
        render_request: &mut crate::render::RenderRequest,
    ) -> ggez::GameResult {
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
        self.events.push(event::Event::MousePress {
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
        self.events.push(event::Event::MouseRelease {
            button,
            position: shared::maths::Point::new(x as f64, y as f64),
        })
    }
    pub fn register_mouse_motion(&mut self, x: f32, y: f32, dx: f32, dy: f32) {
        self.events.push(event::Event::MouseMotion {
            position: shared::maths::Point::new(x as f64, y as f64),
            delta: shared::maths::Vec2::new(dx as f64, dy as f64),
        });
    }
    pub fn register_mouse_wheel(&mut self, x: f32, y: f32) {
        self.events.push(event::Event::MouseWheel {
            delta: shared::maths::Point::new(x as f64, y as f64),
        })
    }
    pub fn register_key_down(&mut self, key: ggez::input::keyboard::KeyInput, repeated: bool) {
        self.events.push(event::Event::KeyDown { key, repeated })
    }
    pub fn register_key_up(&mut self, key: ggez::input::keyboard::KeyInput) {
        self.events.push(event::Event::KeyUp { key })
    }
    pub fn register_text_input(&mut self, character: char) {
        self.events.push(event::Event::TextInput { character })
    }
}

/// Getters
impl UiManager {
    pub fn try_get_element(&mut self, id: Id) -> Option<&mut element::Element> {
        if let Some(index) = self
            .elements
            .iter()
            .enumerate()
            .flat_map(|(i, el)| if el.get_id() == id { Some(i) } else { None })
            .collect::<Vec<usize>>()
            .first()
        {
            Some(self.elements.get_mut(*index).unwrap())
        } else {
            None
        }
    }

    pub fn get_element(&mut self, id: impl Into<Id>) -> &mut element::Element {
        self.try_get_element(id.into()).unwrap()
    }
}
