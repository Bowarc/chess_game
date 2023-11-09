

#[derive(Default)]
pub enum State {
    #[default]
    JustLaunched,
    Disconnected{

    },
    Connecting{
        client: super::Client,
    },
    Connected {
        client: super::Client,
        active_games: crate::networking::Future<Vec<shared::game::Game>>
    },
    Playing {
        client: super::Client,
    },
}