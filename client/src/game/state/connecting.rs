pub struct Connecting {
    ui: crate::ui::UiManager,
    client: crate::game::Client,
}

impl Connecting {
    pub fn new(client: crate::game::Client) -> Self {
        let ui = crate::ui::UiManager::default();
        Self { ui, client }
    }
}

impl super::StateMachine for Connecting {
    fn update(mut self, _: f64) -> super::State {
        if let Err(e) = self.client.update() {
            error!("Error while updating the client in Connecting state: {e}");
            return super::Disconnected::new().into();
        }
        if self.client.is_connected() {
            debug!("Client is now connected, switching State to connected");
            super::Connected::new(self.client).into()
        } else {
            // warn!("Still trying to connect");
            self.into()
        }
    }
    fn draw(mut self, _: &mut crate::render::RenderRequest) -> super::State {
        self.into()
    }

    fn try_get_client_mut(&mut self) -> Option<&mut crate::game::Client> {
        // TODO: Think
        // Do we return the client here, even tho it's proxy is not connected ?
        // I'd say no but the fact that we do have a client and returning None is counter intuitive imo
        // Some(&mut self.client)
        None
    }

    fn try_get_ui_mgr_mut(&mut self) -> Option<&mut crate::ui::UiManager> {
        Some(&mut self.ui)
    }
}
