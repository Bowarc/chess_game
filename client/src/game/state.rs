#[derive(Default)]
pub enum State {
    __Dummy,
    #[default]
    JustLaunched,
    Disconnected{
        ui: crate::ui::UiManager,
    },
    Connecting{
        ui: crate::ui::UiManager,
        client: super::Client,
    },
    Connected {
        ui: crate::ui::UiManager,
        client: super::Client,
        active_games: crate::networking::Future<Vec<shared::game::Game>>
    },
    Playing {
        ui: crate::ui::UiManager,
        client: super::Client,
        current_game: crate::networking::Future<shared::game::Game>,
        current_board: crate::networking::Future<shared::chess::Board>
    },
}

impl State{
    pub fn default_disconnected_ui() -> crate::ui::UiManager{
        crate::ui::UiManager::default()
    }
    pub fn default_connecting_ui() -> crate::ui::UiManager{
        crate::ui::UiManager::default()
        // todo!()
    }
    pub fn default_connected_ui() -> crate::ui::UiManager{
        crate::ui::UiManager::default()
        // todo!()
    }
    pub fn default_playing_ui() -> crate::ui::UiManager{
        crate::ui::UiManager::default()
        // todo!()
    }

}