mod game;
mod player; 
mod state;

pub use game::Game;
pub use player::Player;
pub use state::State;


pub struct GameManager{
    active_games: Vec<game::Game>,
    inactive_players: Vec<player::Player>,
}


impl GameManager{
    pub fn new() -> Self{
        Self {
            active_games: Vec::default(),
            inactive_players: Vec::new(),
        }
    }

    pub fn register_new_game(&mut self, new_game: game::Game){
        self.active_games.push(new_game)
    }

    fn clean_inactive(&mut self){
        let mut i = 0;

        while i < self.active_games.len(){
            
            let game = self.active_games.get_mut(i).unwrap();

            if !game.is_active(){
                println!("Removing game {} koz it's empty", game.id());
                self.active_games.remove(i);
            }else{
                i +=1;
            }
        }
        // self.active_games.retain(|game| game.is_active())
    }


    /// 'Steals' the clients from the server and registers them as player to store them in the list of inactive players
    fn register_new_players(&mut self, server: &mut crate::networking::Server<
        shared::message::ClientMessage,
        shared::message::ServerMessage,
    >){
        let clients_ref = server.clients();

        while let Some(client) = clients_ref.pop(){
            let new_player = Player::new(client);

            debug!("A new player with id: {} has been registered by the game manager", new_player.id());

            self.inactive_players.push(new_player);
        }
    }

    fn update_games(&mut self){
        for game in &mut self.active_games{
            game.update();
        }
    }

    fn update_connected_players(&mut self, ){
        for player in &mut self.inactive_players{
            player.update();
        }
    }

    pub fn update(&mut self, server: &mut crate::networking::Server<shared::message::ClientMessage,shared::message::ServerMessage>){
        self.clean_inactive();
        self.register_new_players(server);
        self.update_games();
    }
}