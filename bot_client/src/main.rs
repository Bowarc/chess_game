use shared::chess;

#[macro_use]
extern crate log;

fn handle_send_error(res: Result<networking::socket::Header, networking::socket::SocketError>) {
    res.unwrap();
}

fn handle_server_message(
    message: shared::message::ServerMessage,
    client: &mut networking::Socket<shared::message::ServerMessage, shared::message::ClientMessage>,
    game: &mut shared::game::Game,
) {
    match message {
        shared::message::ServerMessage::Ping => {
            handle_send_error(client.send(shared::message::ClientMessage::Pong));
        }
        shared::message::ServerMessage::PlayerIdResponse(my_id) => {
            debug!("My id is: {my_id}")
        }
        shared::message::ServerMessage::Games(game_list) => {
            debug!("Game list: {game_list:#?}")
        }
        shared::message::ServerMessage::GameJoin(game) => {
            debug!("{game:?}")
        }
        shared::message::ServerMessage::GameLeave => {
            debug!("You left the game")
        }
        shared::message::ServerMessage::GameJoinFaill(error) => {
            error!("Could not join game due to: {error}")
        }
        shared::message::ServerMessage::GameInfoUpdate(id, new_game) => {
            debug!("Game {id} info got updated: {game:?}");
            *game = new_game;
        }
        shared::message::ServerMessage::GameInfoUpdateFail(id, error) => {
            error!("Info for game {id} could not be updated due to: {error}")
        }
        shared::message::ServerMessage::GameCreateSucess(id) => {
            info!("Created new game with id: {id}");
        }
        shared::message::ServerMessage::GameCreatefail(error) => {
            error!("Could not create game due to: {error}")
        }
        shared::message::ServerMessage::MoveResponse { chess_move, valid } => {
            if valid {
                debug!("The move {chess_move:?} was valid")
            } else {
                debug!("The move {chess_move:?} was invalid")
            }
        }

        _ => (),
    }
}

fn game_state(
    client: &mut networking::Socket<shared::message::ServerMessage, shared::message::ClientMessage>,
    mut game: shared::game::Game,
    bot_id: shared::id::Id,
) -> ! {
    let bot_color = find_bot_color(&game, bot_id);
    let all_pieces = [
        shared::chess::Piece::Pawn,
        shared::chess::Piece::Knight,
        shared::chess::Piece::Bishop,
        shared::chess::Piece::Rook,
        shared::chess::Piece::Queen,
        shared::chess::Piece::King,
    ];
    debug!("Bot is ready");
    loop {
        error!("Frame");
        std::thread::sleep(std::time::Duration::from_secs_f32(0.5));
        if let Ok((_header, message)) = client.try_recv() {
            handle_server_message(message, client, &mut game)
        }
        let shared::game::Game {
            id: _,
            players: _,
            state: shared::game::State::Playing { board },
        } = &mut game
        else {
            todo!()
        };

        if board.next_to_play() != bot_color {
            continue;
        }


        // This is dogshit rn but it's for testing
        'outer: loop {
            let moving_piece = random::pick(&all_pieces);
            let mut all_piece_position = Vec::new();
            {
                for file in 0..8 {
                    for rank in 0..8 {
                        if board.read((file, rank).into()) == Some((bot_color, moving_piece)) {
                            all_piece_position.push((file as i8, rank as i8))
                        }
                    }
                }
            }
            if all_piece_position.is_empty() {
                debug!("No position for piece: {moving_piece}");
                continue;
            }
            let moving_piece_pos = random::pick(&all_piece_position);

            let possible_moves = moving_piece.pseudo_legal_relative_moves();

            let mv = random::pick(possible_moves);

            let bot_chess_move = shared::chess::ChessMove::new(
                shared::chess::Position::from_index(
                    moving_piece_pos.0 as u8,
                    moving_piece_pos.1 as u8,
                )
                .unwrap(),
                {
                    let mut target = moving_piece_pos;
                    match bot_color {
                        chess::Color::Black => {
                            target.0 -= mv.x;
                            target.1 -= mv.y;
                        }
                        chess::Color::White =>{
                            target.0 += mv.x;
                            target.1 += mv.y;
                        },
                    }
                    let target =
                        shared::chess::Position::from_index(target.0 as u8, target.1 as u8);
                    if target.is_none() {
                        continue;
                    }
                    target.unwrap()
                },
                moving_piece,
                bot_color,
            );

            client
                .send(shared::message::ClientMessage::MakeMove(bot_chess_move))
                .unwrap();

            'inner: loop {
                let Ok((_header, msg)) = client.try_recv() else {
                    continue 'inner;
                };

                if let shared::message::ServerMessage::MoveResponse { chess_move, valid } = msg {
                    assert_eq!(bot_chess_move, chess_move);
                    if valid {
                        debug!("Moving {moving_piece} ");
                        break 'outer;
                    } else {
                        warn!("Move wasn't right: {moving_piece}: {chess_move:?}");
                        break;
                    }
                }
            }
        }
    }
}

fn find_bot_color(game: &shared::game::Game, bot_id: shared::id::Id) -> shared::chess::Color {
    game.players
        .iter()
        .flatten()
        .flat_map(|p| if p.id == bot_id { Some(p.color) } else { None })
        .flatten()
        .next()
        .unwrap()
}

fn wait_for_join_confirmation(
    client: &mut networking::Socket<shared::message::ServerMessage, shared::message::ClientMessage>,
) -> shared::game::Game {
    loop {
        let Ok((_header, msg)) = client.try_recv() else {
            continue;
        };

        if let shared::message::ServerMessage::GameJoin(game) = msg {
            break game;
        }
    }
}

fn wait_for_game_info_update(
    client: &mut networking::Socket<shared::message::ServerMessage, shared::message::ClientMessage>,
    game_id: shared::id::Id,
) -> shared::game::Game {
    client
        .send(shared::message::ClientMessage::GameInfoRequest(game_id))
        .unwrap();

    loop {
        let Ok((_header, msg)) = client.try_recv() else {
            continue;
        };

        if let shared::message::ServerMessage::GameInfoUpdate(id, game) = msg {
            assert_eq!(game_id, id);
            break game;
        }
    }
}

fn main() {
    let config = logger::LoggerConfig::new().set_level(log::LevelFilter::Debug);
    logger::init(config, Some("log/bot_client.log"));

    let stream = std::net::TcpStream::connect(shared::DEFAULT_ADDRESS).unwrap();
    stream.set_nonblocking(true).unwrap();
    let mut client = networking::Socket::<
        shared::message::ServerMessage,
        shared::message::ClientMessage,
    >::new(stream);

    let bot_id = {
        client
            .send(shared::message::ClientMessage::MyIdRequest)
            .unwrap();
        loop {
            let Ok((_header, msg)) = client.try_recv() else {
                continue;
            };

            if let shared::message::ServerMessage::PlayerIdResponse(id) = msg {
                break id;
            }
        }
    };

    debug!("Bot id is: {bot_id}");

    loop {
        let user_input = {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();
            buffer.trim().to_string()
        };

        let parts = user_input.split(' ').collect::<Vec<&str>>();
        if parts.is_empty() {
            continue;
        }

        match *parts.first().unwrap() {
            "quit" => break,
            "create" => {
                if let Err(e) = client.send(shared::message::ClientMessage::GameCreateRequest) {
                    error!("Could not create game due to {e}");
                }
                debug!("Wating for server to send game code ..");
                let game = wait_for_join_confirmation(&mut client);
                debug!("Joined game: {game:?}");
                let updated_game = wait_for_game_info_update(&mut client, game.id);
                debug!("Got updated game: {game:?}");
                game_state(&mut client, updated_game, bot_id);
            }
            "join" => {
                let p = parts.get(1).unwrap();
                trace!("parsing '{p}'");
                let game_id = unsafe { shared::id::Id::new_unchecked((*p).parse().unwrap()) };
                if let Err(e) =
                    client.send(shared::message::ClientMessage::GameJoinRequest(game_id))
                {
                    error!("Could not send msg due to: {e}");
                    break;
                }

                let game = wait_for_join_confirmation(&mut client);
                debug!("Joined game: {game:?}");

                let _ = wait_for_game_info_update(&mut client, game_id);
                let _ = wait_for_game_info_update(&mut client, game_id);
                let _ = wait_for_game_info_update(&mut client, game_id);
                let _ = wait_for_game_info_update(&mut client, game_id);
                let updated_game = wait_for_game_info_update(&mut client, game_id);
                debug!("Got updated game: {game:?}");

                game_state(&mut client, updated_game, bot_id);
            }
            _ => warn!("Could not understand your message"),
        }
    }
}
