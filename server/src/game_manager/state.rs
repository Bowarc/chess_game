pub use shared::game::State;

// #[derive(Default)]
// pub enum State {
//     PlayerDisconnected,
//     #[default]
//     Waiting,
//     GameStart,
//     PLaying {
//         // infos about the games / board etc..
//         board: shared::chess::Board
//     },
//     GameEnd{
//         winner: Option<shared::id::Id>
//     }
// }

// impl State {

//     pub fn new_playing() -> Self{
//         Self::PLaying { board: shared::chess::Board::default() }
//     }

// }
