#[derive(Default)]
pub enum State {
    __Dummy,
    #[default]
    JustLaunched,
    Disconnected {
        ui: crate::ui::UiManager,
    },
    Connecting {
        ui: crate::ui::UiManager,
        client: super::Client,
    },
    Connected {
        ui: crate::ui::UiManager,
        client: super::Client,
        active_games: crate::networking::Future<Vec<shared::game::Game>>,
    },
    Playing {
        ui: crate::ui::UiManager,
        client: super::Client,
        current_game: crate::networking::Future<shared::game::Game>,
        current_board: crate::networking::Future<shared::chess::Board>,
    },
}

impl State {
    pub fn new_disconnected() -> Self {
        Self::Disconnected {
            ui: crate::ui::UiManager::default(),
        }
    }
    pub fn new_connecting(client: super::Client) -> Self {
        Self::Connecting {
            ui: crate::ui::UiManager::default(),
            client,
        }
    }
    pub fn new_connected(client: super::Client) -> Self {
        Self::Connected {
            ui: crate::ui::UiManager::default(),
            client,
            active_games: crate::networking::Future::new(
                shared::message::ClientMessage::RequestGames,
                |msg| matches!(msg, shared::message::ServerMessage::Games(_)),
                |msg| {
                    if let shared::message::ServerMessage::Games(g) = msg {
                        g
                    } else {
                        panic!("?? Something went wrong in the validator ");
                    }
                },
            ),
        }
    }
}
