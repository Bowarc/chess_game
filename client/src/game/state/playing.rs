const BOARD_UI_GROUP: &str = "board";
const BOARD_SPRITE_UI_GROUP: &str = "board_sprite";
const BOARD_INDICATOR_GROUP: &str = "indicator";

pub struct Playing {
    ui: crate::ui::UiManager,
    client: crate::game::Client,
    current_game: crate::networking::Future<shared::game::Game>,
    // current_board: crate::networking::Future<shared::chess::Board>,
    current_drag: Option<crate::ui::Id>,
    my_id: shared::id::Id,
}

impl Playing {
    pub fn new(
        client: crate::game::Client,
        game_id: shared::id::Id,
        my_id: shared::id::Id,
    ) -> Self {
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
            current_drag: None,
            my_id,
        }
    }
}

impl super::StateMachine for Playing {
    fn update(mut self, ggctx: &mut ggez::Context, _delta_time: f64) -> super::State {
        self.client.received_msg_mut().clear();
        /* Heavy boilerplate, i don't like it but idk how to do it another way execpt macro but it's a bit overkill */
        if !self.client.is_connected() {
            warn!("Client has been disconnected");
            return super::State::on_disconnect();
        }
        if let Err(e) = self.client.update() {
            error!("Got an error while updating the connection with the server: {e}");
            return super::State::on_disconnect();
        }

        let mut index = 0;
        while let Some(msg) = self.client.received_msg().get(index).cloned() {
            index += 1;
            if let shared::message::ServerMessage::MoveResponse { chess_move, valid } = msg {
                debug!("Move {chess_move:?} validity: {valid}")
            }
        }

        self.current_game.update(&mut self.client);

        let current_game_changed = self.current_game.changed();
        let Some(current_game) = self.current_game.inner_mut() else {
            // The game data has not yet been received
            return self.into();
        };

        // Check if the game state is the correct one
        if !matches!(current_game.state(), &shared::game::State::Playing { .. }) {
            return super::State::from_shared_state(
                self.client,
                self.current_game.inner().cloned().unwrap(),
                self.my_id,
            );
        }

        let shared::game::Game {
            id: _,
            players,
            state: shared::game::State::Playing { board },
        } = /*implicit &mut */ current_game
        else {
            // This should never occur as i check it just above
            panic!()
        };

        let my_color = players
            .iter()
            .flatten()
            .flat_map(|p| {
                if p.id == self.my_id {
                    Some(p.color)
                } else {
                    None
                }
            })
            .flatten()
            .next()
            .unwrap();

        // The game just got received
        if current_game_changed {
            // if self.ui.get_group(BOARD_UI_GROUP).is_none() {
            create_board(&mut self.ui, my_color);

            create_board_pieces(&mut self.ui, board)
            // }
        }

        self.ui.update(ggctx);

        display_move_indicator(&mut self.ui, board, my_color, self.current_drag.as_ref());

        match get_current_move_delta(&mut self.current_drag, &mut self.ui) {
            Ok(Some((start, end))) => 'block: {
                if board.next_to_play() != my_color {
                    warn!("Wait your turn");
                    break 'block; // This is so cool, thanks Mr. Crowley (RFC: label-break-value #2046)
                }
                // Build the chessmove
                let start =
                    shared::chess::Position::from_index(start.0 as u8, start.1 as u8).unwrap();
                let end = shared::chess::Position::from_index(end.0 as u8, end.1 as u8).unwrap();
                debug!("{start} -> {end}",);
                let Some((scolor, spiece)) = board.read(start) else {
                    warn!("{start} is an empty square");
                    break 'block;
                };

                if start == end {
                    warn!("start == end");
                    break 'block;
                }

                // let Some((ecolor, epiece)) = board.read(start) else{
                //     warn!("{end} is an empty square");
                //     break 'block;
                // };

                if let Err(e) = self.client.send(shared::message::ClientMessage::MakeMove(
                    shared::chess::ChessMove::new(start, end, spiece, scolor),
                )) {
                    warn!("Could not send move request to server due to: {e}");
                    break 'block;
                }
            }
            Ok(None) => {
                // No move detected this frame
            }
            Err(e) => {
                error!("An error happened while tring to read the user's move: {e}")
            }
        }

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

fn display_move_indicator(
    ui: &mut crate::ui::UiManager,
    board: &shared::chess::Board,
    color: shared::chess::Color,
    current_drag: Option<&crate::ui::Id>,
) {
    if current_drag.is_none() {
        let _ = ui.remove_group(BOARD_INDICATOR_GROUP);
        return;
    }

    if ui.get_group(BOARD_INDICATOR_GROUP).is_some() {
        return;
    }

    let pos_index = get_pos_from_id(current_drag.unwrap());

    let pos = shared::chess::Position::from_index(pos_index.0 as u8, pos_index.1 as u8).unwrap();
    let pos_index = (pos.file().to_index(), pos.rank().to_index());
    let Some((p_color, piece)) = board.read(pos) else {
        return;
    };

    let mvs = piece.pseudo_legal_relative_moves();

    for mut mv in mvs.clone() {
        if color == shared::chess::Color::Black {
            mv.y *= -1;
        }

        if color != p_color {
            mv.y *= -1;
        }

        let mv_pos = (pos_index.0 as i8 + mv.x, pos_index.1 as i8 + mv.y);
        let id = format!("board_square_{}x{}", mv_pos.0, mv_pos.1);
        let Some(element) = ui.try_get_element(id) else {
            warn!("Skipping {mv:?}");
            continue;
        };

        let el_pos = element.get_pos_value();

        let new_element = crate::ui::element::Element::new_text(
            format!("Indicator{mv:?}"),
            el_pos.clone(),
            20.,
            crate::ui::Style::default(),
            vec![crate::ui::element::TextBit::new_text("", None)],
        );

        ui.add_element(new_element, BOARD_INDICATOR_GROUP);
    }
}

// Returns the chess position (indexes) given by the current square
fn get_pos_from_id(id: &crate::ui::Id) -> (i8, i8) {
    let id = id.replace("board_square_", "").replace('x', "");
    // debug!("{id}");
    // Id should then have a len of 2
    assert_eq!(id.len(), 2);
    let (x, y) = id.split_at(1);
    (x.parse().unwrap(), y.parse().unwrap())
}

/// Retrun delta if a move was played by the player this frame
#[allow(clippy::type_complexity)] // return type is flagged as complex
fn get_current_move_delta(
    current_drag: &mut Option<crate::ui::Id>,
    ui: &mut crate::ui::UiManager,
) -> Result<Option<((i8, i8), (i8, i8))>, String> {
    // Check if the ui is laoded
    if ui.get_group(BOARD_UI_GROUP).is_none() {
        return Err(String::from("A current game is needed for this operation"));
    }

    // Check if there is a current drag
    let Some(dragged_id) = &current_drag else {
        // No id saved, has the user clicked on a square this frame ?

        // For every square, check if it's currently clicked, if so, save the id as currently dragged
        'outer: for y in 0..8 {
            '_inner: for x in 0..8 {
                let square_id = format!("board_square_{y}x{x}");

                // Explicit unwrap is better than implicit
                let element = ui.try_get_element(square_id.clone()).unwrap();

                let element = element.inner_mut::<crate::ui::element::Button>();

                if element.clicked_this_frame() {
                    *current_drag = Some(square_id);
                    break 'outer;
                }
            }
        }
        return Ok(None);
    };

    // Get the currently dragged button
    let element = ui.try_get_element(dragged_id).ok_or(String::from(
        "The saved id doesn't correspond to any element of the current ui",
    ))?;
    let Some(button) = element.try_inner::<crate::ui::element::Button>() else {
        return Err(String::from(
            "The currently saved id is not a button of the current ui",
        ));
    };

    // Here we get the square position
    let (start_x, start_y) = get_pos_from_id(dragged_id);

    if button.get_state().clicked() {
        // Button is not yet released, no move detected
        return Ok(None);
    }

    // Here we know what the user has released the mouse

    // In any case, we need to get rid of that, for example if the user released their mouse outside the chess board
    *current_drag = None;

    // On key release, check the distance between the currenly dragged square and the one where the user released their mouse

    // Loop over pieces to find the square where the mouse is
    for y in 0..8 {
        for x in 0..8 {
            let square_id = format!("board_square_{y}x{x}");

            // TODO: error handleing
            let element2 = ui.try_get_element(square_id.clone()).ok_or(format!(
                "The id ({square_id}) doesn't correspond to any element of the current ui"
            ))?;

            // TODO: error handleing
            let button2 = element2
                .try_inner_mut::<crate::ui::element::Button>()
                .ok_or(format!(
                    "The id ({square_id}) is not a button of the current ui"
                ))?;

            if button2.get_state().hovered() {
                let (end_x, end_y) = get_pos_from_id(&square_id);
                return Ok(Some(((start_x, start_y), (end_x, end_y))));
            }
        }
    }

    Ok(None)
}

fn create_board_pieces(ui: &mut crate::ui::UiManager, board: &shared::chess::Board) {
    use crate::{
        assets::sprite::SpriteId,
        ui::{element::Element, element::TextBit, Style},
    };
    let _ = ui.remove_group(BOARD_SPRITE_UI_GROUP);

    for j in 0..8 {
        for i in 0..8 {
            let position = shared::chess::Position::from_index(i, j).unwrap();
            if let Some((color, piece)) = board.read(position) {
                let sprite_id = SpriteId::ChessPiece(color, piece);

                let id = format!("board_square_{i}x{j}");

                // debug!("Updating square with id: {id}");

                let button = ui.get_element(id);

                let pos = button.get_pos_value();

                let size = button.get_size_value();

                let el = Element::new_image(
                    format!("board_sprite_{i}x{j}"),
                    pos.clone(),
                    size.clone() * 0.8,
                    Style::default(),
                    sprite_id,
                );

                // let el = Element::new_text(
                //     format!("board_sprite_{i}x{j}"),
                //     pos.clone(),
                //     size.x() * 0.8,
                //     Style::default(),
                //     vec![TextBit::new_img(sprite_id, None)],
                // );

                ui.add_element(el, BOARD_SPRITE_UI_GROUP);
            }
        }
    }
}

fn create_board(ui: &mut crate::ui::UiManager, mycolor: shared::chess::Color) {
    use crate::{
        render::Color,
        ui::{element::Element, style, value, Style, Vector},
    };
    let build_style = |main_color: &str| -> style::Bundle {
        style::Bundle::new(
            Style::new(Color::from_hex(main_color), None, None),
            Some(Style::new(
                Color::from_hex(&format!("{main_color}aa")),
                None,
                Some(style::Border::new(Color::from_hex("#000000"), 5.)),
            )),
            Some(Style::new(
                Color::from_hex(&format!("{main_color}55")),
                None,
                Some(style::Border::new(Color::from_hex("#000000"), 5.)),
            )),
        )
    };

    let board_size =
        (value::MagicValue::ScreenSizeW * 0.5 + value::MagicValue::ScreenSizeW * 0.5) * 0.5;

    let square_size = Vector::new(board_size.clone() / 8., board_size / 8.);

    let style1 = build_style("#e3c16f");

    let style2 = build_style("#b88b4a");

    let square_spacing = 0.;

    let _ = ui.remove_group(BOARD_UI_GROUP);

    for i in 0..8 {
        let i = i as f64;
        for j in 0..8 {
            let j = j as f64;

            #[rustfmt::skip]
            let square_center = Vector::new(
                value::MagicValue::ScreenSizeW * 0.5 // Take half the width
                - 8. * 0.5 // Substract haft the number of pieces
                * (square_size.x() + square_spacing) // multiply that by the size of a square + the spacing
                + square_size.x() * 0.5 // Add half the size of a square for make a perfect center
                + (square_size.x() + square_spacing) * i, // Then add the i times the size of a square + the spacing

                value::MagicValue::ScreenSizeH * 0.5 // Take half the height
                - 8. * 0.5 // Substract haft the number of pieces
                * (square_size.y() + square_spacing) // multiply that by the size of a square + the spacing
                + square_size.y() * 0.5 // Add half the size of a square for make a perfect center
                + (square_size.y() + square_spacing) * j, // Then add the i times the size of a square + the spacing
            );

            let id = match mycolor {
                shared::chess::Color::Black => format!("board_square_{}x{j}", 7 - i as u8),
                shared::chess::Color::White => format!("board_square_{i}x{}", 7 - j as u8),
            }; // This is nessesary due to the direction of the board vs the dierction of the nested loop

            let el = Element::new_button(
                id,
                square_center,
                square_size.clone(),
                if (i + j) as i32 % 2 == 0 {
                    style1
                } else {
                    style2
                },
            );

            // Debug, displays the short id
            // debug!("Creating button for square {i}x{j} with id: {id}");
            // let short_id = id.split_at(13).1;

            // let el = Element::new_text(
            //     id.clone(),
            //     square_center,
            //     22.,
            //     Style::new(Color::from_hex("#b88b4a"), None, None),
            //     vec![element::TextBit::new_text(short_id, None)]
            // );

            ui.add_element(el, BOARD_UI_GROUP);
        }
    }
}
