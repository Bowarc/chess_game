mod game;
mod player;
mod state;

pub use game::Game;
pub use player::Player;
pub use state::State;

pub struct GameManager {
    active_games: Vec<game::Game>,
    inactive_players: Vec<player::Player>,
}

impl GameManager {
    pub fn new() -> Self {
        Self {
            active_games: vec![Game::new(), Game::new()],
            inactive_players: Vec::new(),
        }
    }

    pub fn register_new_game(&mut self, new_game: game::Game) {
        self.active_games.push(new_game)
    }

    fn clean_inactive_games(&mut self) {
        let mut i = 0;

        while i < self.active_games.len() {
            let game = self.active_games.get_mut(i).unwrap();

            if !game.is_active() {
                println!("Removing game {} koz it's empty", game.id());
                self.active_games.remove(i);
            } else {
                i += 1;
            }
        }
        // self.active_games.retain(|game| game.is_active())
    }

    fn clean_disconnected_players(&mut self) {
        let mut i = 0;

        while i < self.inactive_players.len() {
            let p = self.inactive_players.get(i).unwrap(); // this shouls be fine
            if !p.is_connected() {
                debug!(
                    "Player ({}) has been removed from the games due to client disonnection",
                    p.id()
                );
                self.inactive_players.remove(i);
            } else {
                i += 1
            };
        }
    }

    /// 'Steals' the clients from the server and registers them as player to store them in the list of inactive players
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

            self.inactive_players.push(new_player);
        }
    }

    fn update_games(&mut self) {
        for game in &mut self.active_games {
            game.update();
        }
    }

    fn update_connected_players(&mut self) {
        // A good idea could be to not compute thye transformation of each games into spimpler game format (shared::game's format)
        // and save them here instead of computing them for each player, that said, i highly doubt that multiple players will be requesting the game list in the same frame

        let mut player_index = 0;

        while player_index < self.inactive_players.len() {

            let Some(player) = self.inactive_players.get_mut(player_index) else{
                error!("Dafuk");
                break;
            };

            debug!("updating player {}", player.id());

            let mut removed = false;
            while let Ok(msg) = player.try_recv() {
                debug!("Received {:?} from ({})", msg, player.id());
                match msg {
                    shared::message::ClientMessage::Text(txt) => debug!(
                        "[Player {}] Sent message of type Text with text: {txt}",
                        player.id()
                    ),
                    shared::message::ClientMessage::RequestGames => {
                        debug!("[Player {}] Requested the list of games", player.id());

                        if let Err(e) = player.send(shared::message::ServerMessage::Games(
                            self.active_games
                                .iter()
                                .map(|game| game.into())
                                .collect::<Vec<shared::game::Game>>(),
                        )) {
                            error!(
                                "[Player {}] Failled to send game list, reason: {e}",
                                player.id()
                            )
                        }
                    }
                    shared::message::ClientMessage::Ping | shared::message::ClientMessage::Pong => {
                        // warn!("[Player {}] Uncaught Ping/Pong message", player.id())
                    }
                    shared::message::ClientMessage::GameJoinRequest(game_id) => {
                        // drop(player);

                        let player_id = player.id();

                        let Some(game_index) = self.active_games.iter().position(|g|g.id() == game_id)else{
                            if let Err(e) = player.send(
                                shared::message::ServerMessage::GameJoinFaill(
                                    "Could not find the requested game".to_string()
                                )
                            ){
                                error!("Could not send game join error to player ({player_id}): {e}")
                            }
                            continue;
                        };

                        let game = self.active_games.get_mut(game_index).unwrap();
                        if game.is_full(){
                            error!("Could not connect player ({player_id}) to game ({game_id}), the game is full");
                            continue;
                        }

                        if let Err(e) = game.connect_player(self.inactive_players.swap_remove(player_index)){
                            error!("Got an error while connecting player ({player_id}) to game ({game_id}): {e}");
                            break;
                        }else{
                            removed = true;
                            break
                        } 
                    }
                    shared::message::ClientMessage::GameInfoRequest(game_id) => {
                        if let Err(e) = player.send(shared::message::ServerMessage::Games(
                            self.active_games
                                .iter()
                                .map(|game| game.into())
                                .collect::<Vec<shared::game::Game>>(),
                        )) {
                            error!(
                                "[Player {}] Failled to send game list, reason: {e}",
                                player.id()
                            )
                        }

                        let Some(game_index) = self.active_games.iter().position(|g|g.id() == game_id)else{
                            error!("Player ({player_id}) requested info on game {game_id} but this game no longer exists", player_id = player.id());
                            if let Err(e) = player.send(
                                shared::message::ServerMessage::GameInfoUpdateFail(game_id, "Could not fetch active game with the give id".to_string())
                            ){
                                error!("Could not inform player ({player_id}) that their request for game ({game_id})'s info failled due to: {e}", player_id = player.id())
                            }
                            
                            break;
                        };
                        if let Err(e) = player.send(
                            shared::message::ServerMessage::GameInfoUpdate(game_id, self.active_games.get(game_index).unwrap().into())
                            ){

                            error!("Player ({player_id}) requested a info update on game ({game_id}) but server failled to send the data: {e}", player_id = player.id())
                        }
                    },
                    shared::message::ClientMessage::GameCreateRequest => {
                        let player_id = player.id();
                        debug!("Player ({player_id}) requested the creation of a game");
                        let mut game = Game::new();
                        if let Err(e) = game.connect_player(self.inactive_players.swap_remove(player_index)){
                            error!("Could not connect player ({player_id}) due to: {e}");
                        }else{
                            self.active_games.push(game);
                        }
                        removed = true;
                        break;

                    }
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
        self.clean_inactive_games();
        self.clean_disconnected_players();
        self.register_new_players(server);
        self.update_connected_players();
        self.update_games();
    }
}
