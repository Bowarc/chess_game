pub struct Playing {
    ui: crate::ui::UiManager,
    client: crate::game::Client,
    current_game: crate::networking::Future<shared::game::Game>,
    // current_board: crate::networking::Future<shared::chess::Board>,
}

impl Playing {
    pub fn new(client: crate::game::Client, game_id: shared::id::Id) -> Self {
        debug!("Creating Playing State");
        Self {
            ui: crate::ui::UiManager::default(),
            client,
            current_game: crate::networking::Future::new(
                shared::message::ClientMessage::GameInfoRequest(game_id),
                |msg| matches!(msg, shared::message::ServerMessage::GameInfoUpdate(..)),
                |msg| {
                    if let shared::message::ServerMessage::GameInfoUpdate(_id, game) = msg {
                        // Cannot capture variables...
                        // if id !=game_id{
                        //     return None
                        // }
                        return Some(game);
                    }
                    None
                },
            ),
        }
    }
}

impl super::StateMachine for Playing {
    fn update(mut self, _ggctx: &mut ggez::Context, _delta_time: f64) -> super::State {
        if !self.client.is_connected(){
            warn!("Client has been disconnected");
            return super::Connecting::new(self.client).into();
        }
        if let Err(e) = self.client.update() {
            error!("Got an error while updating the connection with the server: {e}");
            return super::Connecting::new(self.client).into();
        }

        self.current_game.update(&mut self.client);

        let Some(game) = self.current_game.inner_mut() else{
            debug!("Retu");
            return self.into()
        };

        match game.state(){
            shared::game::State::PlayerDisconnected => {
                // if game.player contains my id
                // But i have no idea what is my id
                // Will later be made using player names so it will be easier 

                debug!("The other player disconnected");
            },
            shared::game::State::Waiting => {
                debug!("Waiting for another player to connect")
            },
            shared::game::State::GameStart => {
                debug!("game is stating! poggers");
            },
            shared::game::State::PLaying { board } =>{
                // debug!("Player turn: {:?}", board.next_to_play());
            } 
            shared::game::State::GameEnd { winner } => {
            },
        }



        // debug!("playing");

        self.into()
        
    }

    fn draw(self, _: &mut crate::render::RenderRequest) -> super::State {
        self.into()
    }

    fn try_get_client_mut(&mut self) -> Option<&mut crate::game::Client> {
        Some(&mut self.client)
    }

    fn try_get_ui_mgr_mut(&mut self) -> Option<&mut crate::ui::UiManager> {
        Some(&mut self.ui)
    }
}
