pub struct Connected {
    ui: crate::ui::UiManager,
    client: crate::game::Client,
    active_games: crate::networking::Future<Vec<shared::game::Game>>,
}

impl Connected {
    pub fn new(client: crate::game::Client) -> Self {
        debug!("Creating Connected State");
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
    fn update_client(mut self) -> Result<Self, super::State> {
        self.client.received_msg_mut().clear();

        if let Err(e) = self.client.update() {
            warn!(
                "Connected state got an error on it's client update: {e}"
            );
            return Err(super::Connecting::new(self.client).into());
        }

        let mut received_game_id = None;

        for msg in self.client.received_msg().clone().iter() {
            match msg {
                // shared::message::ServerMessage::Games(games) => create_games_ui(&mut self.ui, games),
                shared::message::ServerMessage::GameJoinFaill(emsg) => {
                    warn!("Could not join game koz of: {emsg}");
                    // Probably a sync error, let's refresh the games
                    self.active_games.request(&mut self.client).unwrap();
                }
                shared::message::ServerMessage::GameJoin(game) => {
                    debug!("We joined game ({})", game.id());
                    received_game_id = Some(game.id());
                    // Cannot return here as we have a borrow on self.client,
                    // Solutions are:
                    //      Storing optional return value before the loop and breaking
                    //      Cloning the message before looping through them
                    //      Using a while let Some() with .pop on the received messages, but this would require doing that at the end of the client update cycle, therefore having a 1 frame delay on messages
                    break;
                }
                shared::message::ServerMessage::GameInfoUpdateFail(id, emsg) => {
                    warn!("Server failled to send back the data for game {id} due to: {emsg}");
                    // This should never happend here, at least for now
                }
                _ => (),
            }
        }

        if let Some(game_id) = received_game_id {
            return Err(super::Playing::new(self.client, game_id).into());
        }

        Ok(self)
    }

    fn update_ui(mut self, ggctx: &mut ggez::Context) -> Result<Self, super::State> {
        self.ui.update(ggctx);

        self.active_games.update(&mut self.client);

        if self.active_games.changed() && self.active_games.inner().is_some() {
            create_games_ui(&mut self.ui, self.active_games.inner().unwrap());
            debug!("Created new ui for received games");
        }

        // Check if ui has been clicked
        if let Some(active_games) = self.active_games.inner() {
            for game in active_games.iter() {
                let Some(el) = self.ui.try_get_element(format!("Game{}join_button", game.id())).and_then(|el|el.try_inner_mut::<crate::ui::element::Button>()) else{
                    continue;
                };
                if el.clicked_this_frame() {
                    debug!("I wanna connect to game with id: {}", game.id());
                    self.client
                        .send(shared::message::ClientMessage::GameJoinRequest(game.id()))
                        .unwrap();
                }
            }

            if let Some(el) = self
                .ui
                .try_get_element("Game_create_button")
                .and_then(|el| el.try_inner_mut::<crate::ui::element::Button>())
            {
                if el.clicked_this_frame() {
                    debug!("I wanna create a new game");
                    self.client
                        .send(shared::message::ClientMessage::GameCreateRequest)
                        .unwrap();
                }
            }
        }

        Ok(self)
    }
}

impl super::StateMachine for Connected {
    fn update(mut self, ggctx: &mut ggez::Context, _delta_time: f64) -> super::State {
        // if !self.client.is_connected() {
        //     return super::Connecting::new(self.client).into();
        // }

        match self.update_client() {
            Ok(new) => self = new,
            Err(new) => return new,
        }

        match self.update_ui(ggctx) {
            Ok(new) => self = new,
            Err(new) => return new,
        }
        self.into()
    }

    fn draw(self, _: &mut crate::render::RenderRequest) -> super::State {
        self.into()
    }

    fn try_get_client_mut(&mut self) -> Option<&mut crate::game::Client> {
        Some(&mut self.client)
    }

    fn try_get_ui_mgr_mut(&mut self) -> Option<&mut crate::ui::UiManager> {
        Some(&mut self.ui)
    }
}

fn create_games_ui(ui_mgr: &mut crate::ui::UiManager, games: &[shared::game::Game]) {
    use crate::{
        render,
        ui::{self, value::MagicValue},
    };

    let group_name = "game_list_display";

    // Remove all displayed games if there is any
    let _ = ui_mgr.remove_group(group_name);

    let size = ui::Vector::new(MagicValue::ScreenSizeW * 0.3, MagicValue::ScreenSizeH * 0.1);

    // Remember that the position here is the center of the element
    let pos = ui::Vector::new(
        MagicValue::ScreenSizeW * 0.5,
        MagicValue::ScreenSizeH * 0.2 + size.y() * 0.5,
    );

    let style = ui::Style::new(
        render::Color::from_rgba(100, 100, 100, 100),
        Some(ui::style::BackgroundStyle::new(
            render::Color::from_rgb(100, 100, 100),
            None,
        )),
        Some(ui::style::BorderStyle::new(render::Color::random_rgb(), 2.)),
    );

    for (i, game) in games.iter().enumerate() {
        let i = i as f64;
        let offset = ui::Vector::new(0f64, MagicValue::ScreenSizeH * (0.03) * i + size.y() * i);

        let position = pos.clone() + offset.clone();
        let text_size = (size.y() + size.x()) * 0.024;

        ui_mgr.add_element(
            ui::element::Element::new_button(
                format!("Game{i}"),
                position.clone(),
                size.xy(),
                style.into(),
            ),
            group_name,
        );
        ui_mgr.add_element(
            ui::element::Element::new_text(
                format!("Game{i}id_text"),
                position.clone() - size.clone() * 0.4,
                text_size.clone(),
                ui::Style::new(render::Color::random_rgb(), None, None),
                vec![ui::element::TextBit::new_text(
                    format!("Game id: {}", game.id()),
                    Some(render::Color::random_rgb()),
                )],
            ),
            group_name,
        );

        ui_mgr.add_element(
            ui::element::Element::new_text(
                format!("Game{i}player_count_text"),
                position.clone() + (size.x() * 0.4, 0. - size.y() * 0.4),
                text_size,
                ui::Style::new(render::Color::random_rgb(), None, None),
                vec![ui::element::TextBit::new_text(
                    format!("Players: {}/{}", game.player_count(), game.max_players()),
                    Some(render::Color::random_rgb()),
                )],
            ),
            group_name,
        );

        let button_size = (size.y() + size.x()) * 0.08;
        let button_size = ui::Vector::new(button_size.clone(), button_size);
        let join_button_pos = position + size.clone() * 0.5 - button_size.clone() * 0.5;
        ui_mgr.add_element(
            ui::element::Element::new_button(
                format!("Game{}join_button", game.id()),
                join_button_pos.clone(),
                button_size.clone(),
                style.into(),
            ),
            group_name,
        );

        ui_mgr.add_element(
            ui::element::Element::new_text(
                format!("Game{i}join_button_text"),
                join_button_pos,
                (button_size.x() + button_size.y()) * 0.2,
                ui::Style::new(render::Color::default(), None, None),
                vec![ui::element::TextBit::new_text(
                    "Join".to_string(),
                    Some(render::Color::random_rgb()),
                )],
            ),
            group_name,
        );
    }

    // Add a new game button
    let new_b_size = ui::Vector::new(size.x() * 0.3, size.y());
    let pos = ui::Vector::new(
        pos.x() + size.x() * 0.5 + new_b_size.x() * 0.5 + MagicValue::ScreenSizeW * 0.01,
        pos.y(),
    );

    ui_mgr.add_element(
        ui::element::Element::new_button(
            "Game_create_button",
            pos.clone(),
            new_b_size.xy(),
            style.into(),
        ),
        group_name,
    );
    ui_mgr.add_element(
        ui::element::Element::new_text(
            "New game button text",
            pos,
            new_b_size.x() * 0.1,
            ui::Style::new(render::Color::default(), None, None),
            vec![ui::element::TextBit::new_text(
                "Create new",
                Some(render::Color::random_rgb()),
            )],
        ),
        group_name,
    );
}
