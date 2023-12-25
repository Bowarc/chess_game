pub struct Game {
    id: shared::id::Id,
    // player1: Option<super::Player>,
    // player2: Option<super::Player>,
    players: [Option<super::Player>; 2],
    state: super::State,
    lobby_sender: std::sync::mpsc::Sender<super::Player>,
}

impl Game {
    #[allow(clippy::new_without_default)]
    pub fn new(lobby_sender: std::sync::mpsc::Sender<super::Player>) -> Self {
        Self {
            id: shared::id::Id::new(),
            // player1: None,
            // player2: None,
            players: [None, None],
            state: super::State::default(),
            lobby_sender,
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

        debug!(
            "Connected player ({}) to game {}",
            new_player.name(),
            self.id
        );
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

    // Set disconnected player to None
    fn clean_players(&mut self) {
        let mut index = 0;
        while index < self.players.len() {
            if let Some(player) = self
                .players
                .get(index)
                .and_then(|inner_option| inner_option.as_ref())
            {
                if !player.is_connected() {
                    *self.players.get_mut(index).unwrap() = None;
                    // error!("Player is disconnected");
                    self.set_state(super::State::PlayerDisconnected)
                }
            }
            index += 1;
        }
    }
    fn update_state(&mut self) {
        let game_image = shared::game::Game::from(&*self);
        let mut broad_update = false;
        match &mut self.state {
            super::State::PlayerDisconnected => {
                // Explanation of why not `.flatten` can be found at Playing variant match
                for player_opt in self.players.iter_mut() {
                    let Some(_player) = player_opt else {
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

                for (_i, player_opt) in self.players.iter_mut().enumerate() {
                    let Some(player) = player_opt.as_mut() else {
                        // error!("Player {i} is none");
                        continue;
                    };

                    if !player.is_connected() {
                        *player_opt = None;
                        continue;
                    }
                    while let Ok(msg) = player.try_recv() {
                        match msg {
                            shared::message::ClientMessage::GameInfoRequest(id) => {
                                if id == self.id {
                                    if let Err(e) =
                                        player.send(shared::message::ServerMessage::GameInfoUpdate(
                                            self.id,
                                            game_image.clone(),
                                        ))
                                    {
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
                                        shared::message::ServerMessage::GameInfoUpdateFail(
                                            id,
                                            String::from("Wrong game !"),
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
                                }
                            }
                            shared::message::ClientMessage::LeaveGameRequest => {
                                let player_id = player.id();

                                if let Err(e) =
                                    player.send(shared::message::ServerMessage::GameLeave)
                                {
                                    error!("Could not send Gameleave confirmation to player ({}) due to {e}", player_id);
                                }
                                if let Err(e) = self.lobby_sender.send(player_opt.take().unwrap()) {
                                    error!(
                                        "Could not send back player ({}) to lobby due to {e}",
                                        player_id
                                    )
                                }
                                break;
                            }
                            _ => (),
                        }
                    }
                }
            }
            super::State::GameStart => {
                let mut all_colors = vec![shared::chess::Color::Black, shared::chess::Color::White];
                // Explanation of why not `.flatten` can be found at Playing variant match
                for player_opt in self.players.iter_mut() {
                    let Some(player) = player_opt else {
                        self.set_state(super::State::PlayerDisconnected);
                        break;
                    };

                    // Need to assign a color to players
                    let color = random::pick(&all_colors);

                    all_colors.remove(all_colors.iter().position(|c| c == &color).unwrap());

                    player.set_color(color);
                    
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
            super::State::Playing { board } => {
                use shared::message::{ClientMessage, ServerMessage};
                // if let Some(winner_id) = self.winner {
                //     debug!("{winner_id} won");
                //     self.set_state(super::State::Waiting);
                //     return;
                // }

                /*
                    Using `self.players.iter_mut().flatten()` here is tempting but
                    it would open the gate for a problem to happen, what if one of the player is None ?
                    It will never be caught as it's filtered out by flatten, insead just add a let else.
                */

                for player_opt in self.players.iter_mut() {
                    // Check if the player is a Some()
                    let Some(player) = player_opt else {
                        self.set_state(super::State::PlayerDisconnected);
                        break;
                    };

                    let player_id = player.id();

                    while let Ok(msg) = player.try_recv() {
                        match msg {
                            ClientMessage::GameInfoRequest(game_id) => {
                                if game_id != self.id{
                                    //TODO: disconnect player and reset game state
                                    panic!();
                                }
                                if let Err(e) = player.send(
                                    shared::message::ServerMessage::GameInfoUpdate(self.id, game_image.clone())    
                                ){
                                    error!("Failled to send game update to player ({player_id}) due to: {e}")
                                }
                            }
                            ClientMessage::LeaveGameRequest => {

                                if let Err(e) =
                                    player.send(shared::message::ServerMessage::GameLeave)
                                {
                                    error!("Could not send Gameleave confirmation to player ({}) due to {e}", player_id);
                                }
                                if let Err(e) = self.lobby_sender.send(player_opt.take().unwrap()) {
                                    error!(
                                        "Could not send back player ({}) to lobby due to {e}",
                                        player_id
                                    )
                                }
                                break;
                            }
                            ClientMessage::MakeMove(chess_move) => {
                                // Check validity

                                if board.make_move(chess_move).is_err() {
                                    // If the move isn't good, tell the player and go next
                                    if let Err(e) =
                                        player.send(shared::message::ServerMessage::MoveResponse {
                                            chess_move,
                                            valid: false,
                                        })
                                    {
                                        // TODO Fix
                                        panic!("Could not send move upate to player: ({id}), idk what to do: {e}", id = player.id())
                                        // continue;
                                    } else {
                                        broad_update = true;
                                    }
                                }
                            }
                            _ => {
                                // raf + tg
                            }
                        }
                    }
                }

                // Broadcast update
                if broad_update {
                    for player_opt in self.players.iter_mut() {
                        let Some(player) = player_opt else {
                            unimplemented!()
                        };

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
