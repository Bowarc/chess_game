const BOARD_UI_GROUP: &str = "board";
const BOARD_SPRITE_UI_GROUP: &str = "board_sprite";

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

        if self.current_game.changed() {
            let Some(data) = self.current_game.inner() else {
                // TODO: Good error handleing
                panic!("")
            };
            let color = data
                .players()
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

            if let shared::game::State::Playing { board } =
                self.current_game.inner().unwrap().state()
            {
                if self.ui.get_group(BOARD_UI_GROUP).is_none() {
                    create_board(&mut self.ui);

                    // How do i know what player i am
                    create_board_pieces(&mut self.ui, board, color)
                }
            } else {
                return super::State::from_shared_state(
                    self.client,
                    self.current_game.inner().cloned().unwrap(),
                    self.my_id,
                );
            }
        }

        self.ui.update(ggctx);

        match get_current_move_delta(&mut self.current_drag, &mut self.current_game, &mut self.ui) {
            Ok(Some((start, end))) => {
                debug!("{:?} -> {:? }", start, end)
            }
            Ok(None) => {
                // It do be like that sometimes
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

/// Retrun delta if a move was played by the player this frame
#[allow(clippy::type_complexity)] // return type is flagged as complex
fn get_current_move_delta(
    current_drag: &mut Option<crate::ui::Id>,
    current_game: &mut crate::networking::Future<shared::game::Game>,
    ui: &mut crate::ui::UiManager,
) -> Result<Option<((i8, i8), (i8, i8))>, String> {
    // Check that the board is received, we're using the asumpion that if a game is received and it's not the right one, the state machine changes
    if current_game.inner().is_none() {
        return Err(String::from("A current game is needed for this operation"));
    }

    let get_pos_from_id = |id: &crate::ui::Id| -> (i8, i8) {
        let id = id.replace("board_square_", "").replace('x', "");
        // debug!("{id}");
        // Id should then have a len of 2
        assert_eq!(id.len(), 2);
        let (y, x) = id.split_at(1);
        (x.parse().unwrap(), y.parse().unwrap())
    };

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

fn create_board_pieces(
    ui: &mut crate::ui::UiManager,
    board: &shared::chess::Board,
    color: shared::chess::Color,
) {
    use crate::{
        assets::sprite::SpriteId,
        ui::{element::Element, element::TextBit, Style},
    };
    let mut board = board.clone();

    if color == shared::chess::Color::Black {
        board.flip()
    }

    let _ = ui.remove_group(BOARD_SPRITE_UI_GROUP);

    for j in 0..8 {
        for i in 0..8 {
            let position = shared::chess::Position::from_index(i, j).unwrap();
            if let Some((color, piece)) = board.read(position) {
                let sprite_id = SpriteId::ChessPiece(color, piece);

                let id = format!("board_square_{i}x{j}");

                debug!("Updating square with id: {id}");

                let button = ui.get_element(id);

                let pos = button.get_pos_value();

                let size = button.get_size_value();

                let el = Element::new_text(
                    format!("board_sprite_{i}x{j}"),
                    pos.clone(),
                    size.x() * 0.8,
                    Style::default(),
                    vec![TextBit::new_img(sprite_id, None)],
                );

                ui.add_element(el, BOARD_SPRITE_UI_GROUP);
            }
        }
    }
}

fn create_board(ui: &mut crate::ui::UiManager) {
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
                Some(style::BorderStyle::new(Color::from_hex("#000000"), 5.)),
            )),
            Some(Style::new(
                Color::from_hex(&format!("{main_color}55")),
                None,
                Some(style::BorderStyle::new(Color::from_hex("#000000"), 5.)),
            )),
        )
    };

    let board_size =
        (value::MagicValue::ScreenSizeW * 0.5 + value::MagicValue::ScreenSizeW * 0.5) * 0.5;

    let square_size = Vector::new(board_size.clone() / 8., board_size / 8.);

    let style1 = build_style("#b88b4a");

    let style2 = build_style("#e3c16f");

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

            let id = format!("board_square_{i}x{}", 7 - j as u8); // This is nessesary due to the direction of the board vs the dierction of the nested loop

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
