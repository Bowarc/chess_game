pub struct Disconnected {
    ui: crate::ui::UiManager,
}

impl Disconnected {
    pub fn new() -> Self {
        debug!("Creating Disconnected state");
        Self {
            ui: crate::ui::UiManager::default(),
        }
    }
}

impl super::StateMachine for Disconnected {
    fn update(self, _ggctx: &mut ggez::Context, _delta_time: f64) -> super::State {
        if let Ok(client) = crate::game::Client::new(shared::DEFAULT_ADDRESS) {
            return super::Connecting::new(client).into();
        }
        warn!("Could not connect to the sever..");
        self.into()
    }

    fn draw(self, _: &mut crate::render::RenderRequest) -> super::State {
        self.into()
    }

    fn try_get_ui_mgr_mut(&mut self) -> Option<&mut crate::ui::UiManager> {
        Some(&mut self.ui)
    }
}
