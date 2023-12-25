pub struct Connected {
    ui: crate::ui::UiManager,
    client: crate::game::Client,
    active_games: crate::networking::Future<Vec<shared::game::Game>>,
    my_id: crate::networking::Future<shared::id::Id>
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
            my_id: crate::networking::Future::new(
                shared::message::ClientMessage::MyIdRequest,
                |msg| matches!(msg, shared::message::ServerMessage::PlayerIdResponse(_)),
                |msg| {
                    if let shared::message::ServerMessage::PlayerIdResponse(id) = msg {
                        return Some(id);
                    }
                    None
                },
            ),
        }
    }
    fn update_client(mut self) -> Result<Self, super::State> {
        self.client.received_msg_mut().clear();
        
        if !self.client.is_connected() {
            warn!("Client has been disconnected");
            return Err(super::State::on_disconnect());
        }
        if let Err(e) = self.client.update() {
            error!("Got an error while updating the connection with the server: {e}");
            return Err(super::State::on_disconnect());
        }

        let mut index = 0;
        while let Some(msg) = self.client.received_msg().get(index).cloned(){
            index +=1;
            match msg {
                // shared::message::ServerMessage::Games(games) => create_games_ui(&mut self.ui, games),
                shared::message::ServerMessage::GameJoinFaill(emsg) => {
                    warn!("Could not join game koz of: {emsg}");
                    // Probably a sync error, let's refresh the games
                    self.active_games.request(&mut self.client).unwrap();
                }
                shared::message::ServerMessage::GameJoin(game) => {
                    // Cannot return here as we have a borrow on self.client,
                    // Solutions are:
                    //      Storing optional return value before the loop and breaking
                    //      Cloning the message before looping through them
                    //      Using a while let Some() with .pop on the received messages, but this would require doing that at the end of the client update cycle, therefore having a 1 frame delay on messages
                    debug!("We joined a game ({})", game.id());
                    // TODO: Redo this better, i want the unwrap removed
                    return Err(crate::game::state::GameJoin::new(self.client, game, *self.my_id.inner().expect("Should have received by now, ")).into())
                }
                shared::message::ServerMessage::GameInfoUpdateFail(id, emsg) => {
                    warn!("Server failled to send back the data for game {id} due to: {emsg}");
                    // This should never happend here, at least for now
                }
                _ => (),
            }
        }

        Ok(self)
    }

    fn update_ui(mut self, ggctx: &mut ggez::Context) -> Result<Self, super::State> {
        self.ui.update(ggctx);

        self.active_games.update(&mut self.client);
        self.my_id.update(&mut self.client);

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

            if let Some(el) = self
                .ui
                .try_get_element("game_list_refresh_button")
                .and_then(|el| el.try_inner_mut::<crate::ui::element::Button>())
            {
                if el.clicked_this_frame() {
                    debug!("It's refresh time");
                    self.client
                        .send(shared::message::ClientMessage::RequestGames)
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

    // Remove old displayed games if there is any
    let _ = ui_mgr.remove_group(group_name);

    let card_size = ui::Vector::new(MagicValue::ScreenSizeW * 0.3, MagicValue::ScreenSizeH * 0.1);

    // Remember that the position here is the center of the element
    let first_card_pos = ui::Vector::new(
        MagicValue::ScreenSizeW * 0.5,
        MagicValue::ScreenSizeH * 0.2 + card_size.h() * 0.5,
    );

    let card_style = ui::Style::new(
        render::Color::from_rgba(100, 100, 100, 100),
        Some(ui::style::BackgroundStyle::new(
            render::Color::from_rgb(0, 0, 0),
            None,
        )),
        Some(ui::style::BorderStyle::new(render::Color::random_rgb(), 2.)),
    );

    for (i, game) in games.iter().enumerate() {
        let i = i as f64;
        let offset = ui::Vector::new(
            0f64,
            MagicValue::ScreenSizeH * (0.03) * i + card_size.h() * i,
        );

        let card_pos = first_card_pos.clone() + offset.clone();
        let text_size = (card_size.h() + card_size.w()) * 0.024;

        ui_mgr.add_element(
            ui::element::Element::new_button(
                format!("Game{i}"),
                card_pos.clone(),
                card_size.wh(),
                card_style.into(),
            ),
            group_name,
        );
        ui_mgr.add_element(
            ui::element::Element::new_text(
                format!("Game{i}id_text"),
                card_pos.clone() - card_size.clone() * 0.4,
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
                card_pos.clone() + (card_size.w() * 0.4, 0. - card_size.h() * 0.4),
                text_size,
                ui::Style::new(render::Color::random_rgb(), None, None),
                vec![ui::element::TextBit::new_text(
                    format!("Players: {}/{}", game.player_count(), game.max_players()),
                    Some(render::Color::random_rgb()),
                )],
            ),
            group_name,
        );

        let button_size = (card_size.h() + card_size.w()) * 0.08;
        let button_size = ui::Vector::new(button_size.clone(), button_size);
        let join_button_pos = card_pos + card_size.clone() * 0.5 - button_size.clone() * 0.5;
        ui_mgr.add_element(
            ui::element::Element::new_button(
                format!("Game{}join_button", game.id()),
                join_button_pos.clone(),
                button_size.clone(),
                card_style.into(),
            ),
            group_name,
        );

        ui_mgr.add_element(
            ui::element::Element::new_text(
                format!("Game{i}join_button_text"),
                join_button_pos,
                (button_size.w() + button_size.h()) * 0.2,
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
    let new_b_size = ui::Vector::new(card_size.w() * 0.3, card_size.h());
    let new_b_pos = ui::Vector::new(
        first_card_pos.x()
            + card_size.w() * 0.5
            + new_b_size.w() * 0.5
            + MagicValue::ScreenSizeW * 0.01,
        first_card_pos.y(),
    );

    ui_mgr.add_element(
        ui::element::Element::new_button(
            "Game_create_button",
            new_b_pos.clone(),
            new_b_size.wh(),
            card_style.into(),
        ),
        group_name,
    );
    ui_mgr.add_element(
        ui::element::Element::new_text(
            "New game button text",
            new_b_pos,
            new_b_size.w() * 0.1,
            ui::Style::new(render::Color::default(), None, None),
            vec![ui::element::TextBit::new_text(
                "Create new",
                Some(render::Color::random_rgb()),
            )],
        ),
        group_name,
    );

    // Adding a refresh button
    let refresh_button_size = ui::Vector::new(card_size.x() * 0.1, card_size.x() * 0.1);
    let refresh_button_vertical_margin =
        ui::Vector::new(0., card_size.h() * 0.1);
    let refresh_button_pos = ui::Vector::new(
        first_card_pos.x() + card_size.w() * 0.5 - refresh_button_size.w() * 0.5,
        first_card_pos.y() - card_size.h() * 0.5 - refresh_button_size.h() * 0.5,
    ) - refresh_button_vertical_margin;

    debug!("{card_size:?}");
    ui_mgr.add_element(
        ui::element::Element::new_button(
            "game_list_refresh_button",
            refresh_button_pos.clone(),
            refresh_button_size.wh(),
            card_style.into(),
        ),
        group_name,
    );
    ui_mgr.add_element(
        ui::element::Element::new_text(
            "game_list_refresh_text",
            refresh_button_pos,
            refresh_button_size.w() * 0.25,
            ui::Style::new(render::Color::default(), None, None),
            vec![ui::element::TextBit::new_text(
                "refresh",
                Some(render::Color::random_rgb()),
            )],
        ),
        group_name,
    );
}
