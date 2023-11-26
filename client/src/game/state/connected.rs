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
        if self.active_games.changed() && self.active_games.inner().is_some() {
            create_games_ui(&mut self.ui, self.active_games.inner().unwrap());
            debug!("Created new ui for received games");
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

    let size= ui::Vector::new(MagicValue::ScreenSizeW * 0.3, MagicValue::ScreenSizeH * 0.1);
    // Remember that the position here is the center of the element
    let pos = ui::Vector::new(MagicValue::ScreenSizeW * 0.5, MagicValue::ScreenSizeH * 0.2 + size.y() * 0.5);

    let style = ui::Style::new(
        render::Color::from_rgba(100, 100, 100, 100),
        Some(ui::style::BackgroundStyle::new(
            render::Color::from_rgb(100, 100, 100),
            None,
        )),
        Some(ui::style::BorderStyle::new(render::Color::random_rgb(), 2.)),
    );

    for (i, game) in games.iter().enumerate() {
        let  i = i as f64;
        let id = format!("Game{i}");
        let offset = ui::Vector::new(0f64, MagicValue::ScreenSizeH * (0.03) * i  + size.y() * i );

        let position = pos.clone() + offset;

        let el = ui::element::Element::new_button(
            id,
            position,
            size.xy(),
            style.into(),
        );

        ui_mgr.add_element(el, group_name);

        let topleft = (pos.x() - size.x() *0.5, pos.y() - size.y()*0.5);


    }

    // Add a new game button
    let new_b_size = ui::Vector::new(size.x() * 0.3, size.y());
    let pos = ui::Vector::new(pos.x() + size.x() * 0.5 + new_b_size.x() *0.5 + MagicValue::ScreenSizeW *0.01, pos.y());
    let new_game_button = ui::element::Element::new_button(
        "New game button",
        pos,
        new_b_size.xy(),
        style.into(),
    );
    ui_mgr.add_element(new_game_button, group_name);
}
