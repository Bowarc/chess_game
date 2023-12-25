mod game;
mod player;
mod state;

pub use game::Game;
pub use player::Player;
pub use state::State;

pub struct GameManager {
    games: Vec<game::Game>,
    players: Vec<player::Player>, // every player that is connected to this server

    // used to send back player to the lobby
    lobby_receiver: std::sync::mpsc::Receiver<Player>,
    lobby_sender: std::sync::mpsc::Sender<Player>,

}

impl GameManager {
    pub fn new() -> Self {
        let (sender, receiver) = std::sync::mpsc::channel::<Player>();

        Self {
            games: Vec::new(),
            players: Vec::new(),
            lobby_receiver: receiver,
            lobby_sender: sender,
        }
    }

    fn create_new_game(&mut self) -> &mut Game{
        self.games.push(Game::new(self.lobby_sender.clone()));
        self.games.last_mut().unwrap() // Unless big problem, this will never panic 
    }

    fn clean_inactive_games(&mut self) {
        let mut i = 0;

        while i < self.games.len() {
            let game = self.games.get_mut(i).unwrap();

            if !game.is_active() {
                debug!("Deleting game {} (Empty)", game.id());
                drop(self.games.remove(i));
            } else {
                i += 1;
            }
        }
        // self.games.retain(|game| game.is_active())
    }

    fn clean_disconnected_players(&mut self) {
        let mut i = 0;

        while i < self.players.len() {
            let p = self.players.get(i).unwrap(); // this should be fine
            if !p.is_connected() {
                debug!(
                    "Player ({}) has been removed due to client disonnection",
                    p.id()
                );
                self.players.remove(i);
            } else {
                i += 1
            };
        }
    }

    /// Retrieve players sent back to the lobby by games
    fn catch_returning_players(&mut self){
        while let Ok(player) = self.lobby_receiver.try_recv(){
            debug!("Player ({}) has been retrieved by the game manager", player.id());
            self.players.push(player);
        }
    }

    /// 'Steals' the clients from the server and registers them as player to store them in the list of players
    fn register_new_players(
        &mut self,
        server: &mut crate::networking::Server<
            shared::message::ClientMessage,
            shared::message::ServerMessage,
        >,
    ) {
        let clients_ref = server.clients();

        while let Some(client) = clients_ref.pop() {
            let new_player = Player::new(client);

            debug!(
                "A new player with id: {} has been registered by the game manager",
                new_player.id()
            );

            self.players.push(new_player);
        }
    }

    fn update_games(&mut self) {
        for game in &mut self.games {
            game.update();
        }
    }

    fn update_connected_players(&mut self) {
        // A good idea could be to not compute the transformation of each games into simpler game format (shared::game's format)
        // and save them here instead of computing them for each player, that said, i highly doubt that multiple players will be requesting the game list in the same frame

        let mut player_index = 0;

        // Loop over all players
        while player_index < self.players.len() {
            let Some(player) = self.players.get_mut(player_index) else{
                break;
            };

            let player_id = player.id();

            // debug!("updating player {}", player.id());

            // Players are able to be moved out of the `self.incative_player` list, so we need to keep track of that
            let mut removed = false;
            
            // Loop over every message that the given player sent
            while let Ok(msg) = player.try_recv() {
                // debug!("Received {:?} from ({})", msg, player.id());
                match msg {
                    shared::message::ClientMessage::MyIdRequest => {
                        if let Err(e) = player.send(
                            shared::message::ServerMessage::PlayerIdResponse(player_id)

                        ){
                            // TODO: Error handleing
                            panic!("Couldn't send player ({player_id}) id confirmation message")
                        }
                    }
                    shared::message::ClientMessage::Ping | shared::message::ClientMessage::Pong => {
                        // warn!("[Player {}] Uncaught Ping/Pong message", player.id())
                    }
                    shared::message::ClientMessage::Text(txt) => {
                        debug!("[Player {}] Sent text: {txt}", player.id())
                    }
                    shared::message::ClientMessage::RequestGames => {
                        debug!("[Player {}] Requested the list of games", player.id());

                        if let Err(e) = player.send(shared::message::ServerMessage::Games(
                            self.games
                                .iter()
                                .map(|game| game.into())
                                .collect::<Vec<shared::game::Game>>(),
                        )) {
                            error!(
                                "[Player {player_id}] Failled to send game list, reason: {e}",
                            )
                        }
                    }
                    shared::message::ClientMessage::GameJoinRequest(game_id) => {

                        // Get the requested game index or continue
                        let Some(game_index) = self.games.iter().position(|g|g.id() == game_id) else {
                            if let Err(e) = player.send(
                                shared::message::ServerMessage::GameJoinFaill(
                                    "Could not find the requested game".to_string()
                                )
                            ){
                                error!("Could not send game join error to player ({player_id}): {e}")
                            }
                            continue;
                        };

                        // Get the mut game from the index
                        let game = self.games.get_mut(game_index).unwrap();
                        if game.is_full() {
                            error!("Could not connect player ({player_id}) to game ({game_id}), the game is full");
                            if let Err(e) = player.send(shared::message::ServerMessage::GameJoinFaill("Could not connect to game {game_id}: This game is full".to_string())){
                                error!("Could not send game join error to player ({player_id}): {e}")
                            } 
                            continue;
                        }

                        // Here it's fine to use swap remove as the index doesn't move 
                        // We only lose the player list order, which isn't important imo
                        let moved_player =self.players.swap_remove(player_index);
                        // Once the player is removed, we can't use continue anymore, as the next call to `player.try_recv()` would call a moved value
                        removed = true; 

                        if let Err(e) =
                            game.connect_player(moved_player)
                        {
                            error!("Got an error while connecting player ({player_id}) to game ({game_id}): {e}");
                            break;
                        }

                        break
                    }
                    shared::message::ClientMessage::GameInfoRequest(game_id) => {
                        // What ?
                        // if let Err(e) = player.send(shared::message::ServerMessage::Games(
                        //     self.games
                        //         .iter()
                        //         .map(|game| game.into())
                        //         .collect::<Vec<shared::game::Game>>(),
                        // )) {
                        //     error!(
                        //         "[Player {}] Failled to send game list, reason: {e}",
                        //         player.id()
                        //     )
                        // }

                        let Some(game_index) = self.games.iter().position(|g|g.id() == game_id)else{
                            error!("Player ({player_id}) requested info on game {game_id} but this game no longer exists", player_id = player.id());
                            if let Err(e) = player.send(
                                shared::message::ServerMessage::GameInfoUpdateFail(game_id, "Could not fetch active game with the give id".to_string())
                            ){
                                error!("Could not inform player ({player_id}) that their request for game ({game_id})'s info failled due to: {e}", player_id = player.id())
                            }
                            break;
                        };
                        if let Err(e) = player.send(shared::message::ServerMessage::GameInfoUpdate(
                            game_id,
                            self.games.get(game_index).unwrap().into(),
                        )) {
                            error!("Player ({player_id}) requested a info update on game ({game_id}) but server failled to send the data: {e}", player_id = player.id())
                        }
                    }
                    shared::message::ClientMessage::GameCreateRequest => {
                        debug!("Player ({player_id}) requested the creation of a game");
                        let moved_player = self.players.swap_remove(player_index);

                        let game = self.create_new_game();

                        // Here it's fine to use swap remove as the index doesn't move 
                        // We only lose the player list order, which isn't important imo
                        // Once the player is removed, we can't use continue anymore, as the next call to `player.try_recv()` would call a moved value
                        removed = true; 

                        if let Err(e) =
                            game.connect_player(moved_player)
                        {
                            error!("Could not connect player ({player_id}) due to: {e}");
                        }

                        break;
                    },
                    shared::message::ClientMessage::LeaveGameRequest => {
                        // The player is not in a game, but i can see a world where it's just states that are not synched
                        // So let's just fix that by fake removing it from an imaginary game
                        if let Err(e) = player.send(shared::message::ServerMessage::GameLeave){
                            error!("Could not send Gameleave confirmation to player ({player_id}) due to {e}");

                        }
                    }
                    shared::message::ClientMessage::MakeMove(mv) => {
                        // Must be a desync, try to resync it ? 
                        // Send a move denied and game leave message
                        if let Err(e) = player.send(
                            shared::message::ServerMessage::MoveResponse { chess_move: mv, valid: false }
                        ).and_then(|_| player.send(shared::message::ServerMessage::GameLeave)){

                            error!("Could not send error msg to client ({player_id}) due to: {e}")
                        }
                    },


                }
            }
            if !removed {
                player_index += 1;
            }
        }
    }

    pub fn update(
        &mut self,
        server: &mut crate::networking::Server<
            shared::message::ClientMessage,
            shared::message::ServerMessage,
        >,
    ) {
        self.catch_returning_players();
        self.clean_inactive_games();
        self.clean_disconnected_players();
        self.register_new_players(server);
        self.update_connected_players();
        self.update_games();
    }
}
