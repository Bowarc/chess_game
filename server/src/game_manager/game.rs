pub struct Game {
    id: shared::id::Id,
    // player1: Option<super::Player>,
    // player2: Option<super::Player>,

    players: [Option<super::Player>; 2],

    state: super::State,
}

impl Game {
    pub fn new() -> Self {
        Self {
            id: shared::id::Id::new(),
            // player1: None,
            // player2: None,
            players: Default::default(),
            state: super::State::default(),
        }
    }

    pub fn id(&self) -> shared::id::Id {
        self.id
    }

    pub fn connect_player(
        &mut self,
        mut new_player: super::Player,
    ) -> Result<(), shared::error::server::GameError> {
        debug!("Connected player ({}) to game {}", new_player.id(), self.id);

        if self.is_full(){
            return Err(shared::error::server::GameError::SessionIsFull);
        }

        if let Err(e) = new_player.send(
            shared::message::ServerMessage::GameJoin(self.into())
        ){
            error!("Failled to send connection confirmation to player ({}): {e}", new_player.id());
            return Err(shared::error::server::GameError::FailledToAcceptPlayer)
        }

        for player_opt in &mut self.players{
            if player_opt.is_none(){
                *player_opt = Some(new_player);
                break
            }
        }

        Ok(())
    }

    pub fn is_active(&self) -> bool {
        // self.player1.is_some() || self.player2.is_some()
        self.players.iter().any(|player| player.is_some())
    }

    pub fn is_full(&self) -> bool{
        // self.player1.is_some() && self.player2.is_some()
        !self.players.iter().any(|player| player.is_none())
    }

    pub fn update(&mut self) {
        match self.state{
            super::State::Waiting => {
                // Just wait for new player, maybe make a chat box
                for player_opt in &mut self.players {
                    if let Some(player) = player_opt.as_mut() {
                        if !player.is_connected() {
                            *player_opt = None;
                            continue;
                        }
                        while let Ok(msg) = player.try_recv(){
                            // debug!("player ({}) sent {:?}", player.id(), msg)
                        }
                    }
                }
            },
            super::State::PLaying {  } => {
                let game_image = shared::game::Game::from(&*self);
                for player in self.players.iter_mut().flatten(){
                    if let Err(e) = player.send(
                        shared::message::ServerMessage::GameInfoUpdate(self.id, game_image.clone())
                    ){
                        error!("Game {} failled to comunicate with player ({}): {e}", self.id, player.id())
                    }
                }
            },
        }
        // if let Some(p) = &mut self.player1{
        //     self.update_player(p);
        // }

    }
}

impl From<&Game> for shared::game::Game {
    fn from(server_game: &Game) -> Self {
        shared::game::Game::new(
            server_game.id(),
            server_game.players.iter().map(|p_opt| p_opt.as_ref().map(|p|p.into())).collect::<Vec<Option<shared::game::Player>>>().try_into().unwrap(),
            // server_game.player1.as_ref().map(|player| player.into()),
            // server_game.player2.as_ref().map(|player| player.into()),
        )
    }
}

impl From<&mut Game> for shared::game::Game {
    fn from(server_game: &mut Game) -> Self {
        shared::game::Game::new(
            server_game.id(),
            server_game.players.iter().map(|p_opt| p_opt.as_ref().map(|p|p.into())).collect::<Vec<Option<shared::game::Player>>>().try_into().unwrap(),
        )
    }
}

