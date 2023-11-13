pub struct Playing {
    ui: crate::ui::UiManager,
    client: crate::game::Client,
    current_game: crate::networking::Future<shared::game::Game>,
    current_board: crate::networking::Future<shared::chess::Board>,
}

impl super::StateMachine for Playing {
    fn update(mut self, delta_time: f64) -> super::State {
        if let Err(e) = self.client.update() {
            error!("Got an error while updating the connection with the server: {e}");
            super::Disconnected::new().into()
        } else {
            self.into()
        }
    }

    fn draw(mut self, _: &mut crate::render::RenderRequest) -> super::State {
        self.into()
    }

    fn try_get_client_mut(&mut self) -> Option<&mut crate::game::Client> {
        Some(&mut self.client)
    }

    fn try_get_ui_mgr_mut(&mut self) -> Option<&mut crate::ui::UiManager> {
        Some(&mut self.ui)
    }
}
