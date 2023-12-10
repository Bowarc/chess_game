pub struct Game {
    id: shared::id::Id,
    // player1: Option<super::Player>,
    // player2: Option<super::Player>,
    players: [Option<super::Player>; 2],
    state: super::State,
}

impl Game {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            id: shared::id::Id::new(),
            // player1: None,
            // player2: None,
            players: [None, None],
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
        if self.is_full() {
            return Err(shared::error::server::GameError::SessionIsFull);
        }

        if let Err(e) = new_player.send(shared::message::ServerMessage::GameJoin(self.into())) {
            error!(
                "Failled to send connection confirmation to player ({}): {e}",
                new_player.id()
            );
            return Err(shared::error::server::GameError::FailledToAcceptPlayer);
        }

        debug!("Connected player ({}) to game {}", new_player.name(), self.id);
        for player_opt in &mut self.players {
            if player_opt.is_none() {
                *player_opt = Some(new_player);
                break;
            }
        }

        if self.is_full() {
            self.set_state(super::State::GameStart);
        }

        Ok(())
    }

    pub fn is_active(&self) -> bool {
        // self.player1.is_some() || self.player2.is_some()
        self.players.iter().any(|player| player.is_some())
    }

    pub fn is_full(&self) -> bool {
        // self.player1.is_some() && self.player2.is_some()
        !self.players.iter().any(|player| player.is_none())
    }

    fn set_state(&mut self, new_state: super::State) {
        debug!("Game {} state -> {:?}", self.id, new_state.variant_name());
        self.state = new_state;
        let game_image = shared::game::Game::from(&*self);
        for player in self.players.iter_mut().flatten() {
            if let Err(_e) = player.send(shared::message::ServerMessage::GameInfoUpdate(
                self.id,
                game_image.clone(),
            )) {
                // error!(
                //     "Game {} failled to comunicate with player {} due to {e}",
                //     self.id,
                //     player.id()
                // )
            }
        }
    }

    pub fn update(&mut self) {
        self.clean_players();
        self.update_state();
    }

    fn clean_players(&mut self){
        let mut index= 0;
        while index < self.players.len(){
            if let Some(player) = self.players.get(index).and_then(|inner_option| inner_option.as_ref()){
                if !player.is_connected(){
                    *self.players.get_mut(index).unwrap() = None;
                    self.set_state(super::State::PlayerDisconnected)
                }
            }
            index += 1;
        }
    }
    fn update_state(&mut self){
        match &mut self.state {
            super::State::PlayerDisconnected => {
                // Explanation of why not `.flatten` can be found at Playing variant match
                for player_opt in self.players.iter_mut(){ 
                    let Some(_player) = player_opt else{
                        // self.set_state(super::State::PlayerDisconnected);
                        break;
                    };
                    // "A player has disconnected"
                    // Maybe give them the win ?
                }

                self.set_state(super::State::Waiting)
            }

            super::State::Waiting => {
                // Just wait for new player
                let game_image = shared::game::Game::from(&*self);

                for player_opt in &mut self.players {
                    if let Some(player) = player_opt.as_mut() {
                        if !player.is_connected() {
                            *player_opt = None;
                            continue;
                        }
                        while let Ok(msg) = player.try_recv() {
                            match msg {
                                shared::message::ClientMessage::Text(_)
                                | shared::message::ClientMessage::Ping
                                | shared::message::ClientMessage::Pong
                                | shared::message::ClientMessage::RequestGames
                                | shared::message::ClientMessage::GameJoinRequest(_)
                                | shared::message::ClientMessage::GameCreateRequest => (),
                                shared::message::ClientMessage::GameInfoRequest(id) => {
                                    if id == self.id {
                                        if let Err(e) = player.send(
                                            shared::message::ServerMessage::GameInfoUpdate(
                                                self.id,
                                                game_image.clone(),
                                            ),
                                        ) {
                                            error!(
                                                "Game {} failled to comunicate with player ({}): {e}",
                                                self.id,
                                                player.id()
                                            );
                                            *player_opt = None;
                                            break;
                                        }
                                    } else {
                                        /*
                                            Player requested info about a different game.
                                            What do we do ..?
                                            we could just disconnect that player by dropping it
                                        */
                                        if let Err(e) = player.send(
                                            shared::message::ServerMessage::GameInfoUpdateFail(id, String::from("Wrong game !")),
                                        ) {
                                            error!(
                                                "Game {} failled to comunicate with player ({}): {e}",
                                                self.id,
                                                player.id()
                                            );
                                            *player_opt = None;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            super::State::GameStart => {
                let game_image = shared::game::Game::from(&*self);

                // Explanation of why not `.flatten` can be found at Playing variant match
                for player_opt in self.players.iter_mut(){ 
                    let Some(player) = player_opt else{
                        self.set_state(super::State::PlayerDisconnected);
                        break;
                    };
                    if let Err(e) = player.send(shared::message::ServerMessage::GameInfoUpdate(
                        self.id,
                        game_image.clone(),
                    )) {
                        error!(
                            "Game {} failled to comunicate with player ({}): {e}",
                            self.id,
                            player.id()
                        )
                    }
                }

                self.set_state(super::State::Playing {
                    board: shared::chess::Board::default(),
                });
            }
            super::State::Playing { board: _ } => {
                use shared::message::{ClientMessage, ServerMessage};
                // if let Some(winner_id) = self.winner {
                //     debug!("{winner_id} won");
                //     self.set_state(super::State::Waiting);
                //     return;
                // }
                let game_image = shared::game::Game::from(&*self);
                /*
                    Using `self.players.iter_mut().flatten()` here is tempting but
                    it would open the gate for a problem to happend, what if one of the player is None ?
                    It will never be caught as it's filtered out by flatten, insead just add a let else.
                */
                for player_opt in self.players.iter_mut(){ 
                    let Some(player) = player_opt else{
                        self.set_state(super::State::PlayerDisconnected);
                        break;
                    };


                    // Send game update to each player
                    if let Err(e) =
                        player.send(ServerMessage::GameInfoUpdate(self.id, game_image.clone()))
                    {
                        error!(
                            "Game {} failled to comunicate with player ({}): {e}",
                            self.id,
                            player.id()
                        );
                        self.set_state(super::State::PlayerDisconnected);
                        return;
                    }

                    while let Ok(msg) = player.try_recv() {
                        match msg {
                            ClientMessage::Text(_)
                            | ClientMessage::Ping
                            | ClientMessage::Pong
                            | ClientMessage::RequestGames
                            | ClientMessage::GameJoinRequest(_)
                            | ClientMessage::GameInfoRequest(_)
                            | ClientMessage::GameCreateRequest => {
                                // raf + tg
                            } // ClientMessage::MakeMove(game_id, chess_move) =>  {

                              // }
                        }
                    }
                }
            }

            super::State::GameEnd { winner: _ } => {}
        }
    }
}

impl From<&Game> for shared::game::Game {
    fn from(server_game: &Game) -> Self {
        shared::game::Game::new(
            server_game.id(),
            server_game
                .players
                .iter()
                .map(|p_opt| p_opt.as_ref().map(|p| p.into()))
                .collect::<Vec<Option<shared::game::Player>>>()
                .try_into()
                .unwrap(),
            server_game.state.clone(),
        )
    }
}

impl From<&mut Game> for shared::game::Game {
    fn from(server_game: &mut Game) -> Self {
        shared::game::Game::new(
            server_game.id(),
            server_game
                .players
                .iter()
                .map(|p_opt| p_opt.as_ref().map(|p| p.into()))
                .collect::<Vec<Option<shared::game::Player>>>()
                .try_into()
                .unwrap(),
            server_game.state.clone(),
        )
    }
}
