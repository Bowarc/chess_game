pub struct WaitingForOpponent {
    client: crate::game::Client,
    current_game: crate::networking::Future<shared::game::Game>,
    my_id: shared::id::Id,
}

impl WaitingForOpponent {
    pub fn new(client: crate::game::Client, game: shared::game::Game, my_id: shared::id::Id) -> Self {
        debug!("Creating WaitingForOpponent State");
        Self {
            client,
            current_game: crate::networking::Future::new(
                shared::message::ClientMessage::GameInfoRequest(game.id()),
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
            my_id
        }
    }
}

impl super::StateMachine for WaitingForOpponent {
    fn update(mut self, _ggctx: &mut ggez::Context, _delta_time: f64) -> super::State {
        /* Heavy boilerplate, i don't like it but idk how to do it another way execpt macro but it's a bit overkill */
        if !self.client.is_connected() {
            warn!("Client has been disconnected");
            return super::State::on_disconnect();
        }
        if let Err(e) = self.client.update() {
            error!("Got an error while updating the connection with the server: {e}");
            return super::State::on_disconnect();
        }

        self.current_game.update(&mut self.client);

        if self.current_game.changed()
            && !matches!(
                self.current_game.inner().unwrap().state(),
                &shared::game::State::Waiting
            )
        {
            return super::State::from_shared_state(
                self.client,
                self.current_game.inner().cloned().unwrap(),
                self.my_id
            );
        }

        self.into()
    }

    fn draw(self, _: &mut crate::render::RenderRequest) -> super::State {
        // Waiting for an opponent
        debug!("Waiting for an opponent");
        self.into()
    }
}
