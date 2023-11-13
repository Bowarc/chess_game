pub struct Disconnected {
    ui: crate::ui::UiManager,
}

impl Disconnected {
    pub fn new() -> Self {
        Self {
            ui: crate::ui::UiManager::default(),
        }
    }
}

impl super::StateMachine for Disconnected {
    fn update(mut self, delta_time: f64) -> super::State {
        if let Ok(client) = crate::game::Client::new(shared::DEFAULT_ADDRESS) {
            super::Connecting::new(self.ui, client).into()
        } else {
            warn!("Could not connect to the sever..");
            self.into()
        }
    }

    fn draw(mut self, _: &mut crate::render::RenderRequest) -> super::State {
        self.into()
    }

    fn try_get_ui_mgr_mut(&mut self) -> Option<&mut crate::ui::UiManager> {
        Some(&mut self.ui)
    }
}
