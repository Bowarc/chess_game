pub struct Connected {
    ui: crate::ui::UiManager,
    client: crate::game::Client,
    active_games: crate::networking::Future<Vec<shared::game::Game>>,
}

impl Connected {
    pub fn new(client: crate::game::Client) -> Self {
        debug!("Creating Conncted State");
        Self {
            ui: crate::ui::UiManager::default(),
            client,
            active_games: crate::networking::Future::new(
                shared::message::ClientMessage::RequestGames,
                |msg| matches!(msg, shared::message::ServerMessage::Games(_)),
                |msg| {
                    if let shared::message::ServerMessage::Games(games) = msg {
                        return Some(games);
                    }
                    None
                },
            ),
        }
    }
}

impl super::StateMachine for Connected {
    fn update(mut self, delta_time: f64) -> super::State {
        // For clarity
        if !self.client.is_connected() {
            return super::Connecting::new(self.client).into();
        }
        if let Err(e) = self.client.update() {
            return super::Connecting::new(self.client).into();
        }

        self.active_games.update(&mut self.client);
        if self.active_games.changed() {
            let text_id = self.ui.add_element(crate::ui::element::Element::new_text(
                "Test text 1",
                crate::ui::Anchor::TopCenter,
                20.,
                crate::ui::Style::default(),
                vec![crate::ui::element::TextBit::new_text("Salut", None)],
            ));
            debug!("Added new text with id: {text_id}");
        }
        self.into()
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
