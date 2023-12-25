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
                .next().unwrap();

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
                    self.my_id
                );
            }
        }

        self.ui.update(ggctx);

        if self.current_game.inner().is_some() {
            // Handle ui events

            // hmm
            // First we need to check if a tile is clicked or dragged

            let get_pos_from_id = |id: &crate::ui::Id| -> (i8, i8) {
                let id = id.replace("board_square_", "").replace('x', "");
                debug!("{id}");
                assert_eq!(id.len(), 2);
                // Id should then have a len of 2
                let (y, x) = id.split_at(1);
                (x.parse().unwrap(), y.parse().unwrap())
            };

            if let Some(id) = &self.current_drag {
                // Get the currently dragged button
                let element = self.ui.get_element(id);
                let Some(button) = element.try_inner::<crate::ui::element::Button>() else {
                    panic!("Hmmm not good it is");
                    // return self.into();
                };
                let (x, y) = get_pos_from_id(id);

                // Find a way to get the chess move resulting

                // Maybe ok key release, check the currently hovered tile ?
                if !button.get_state().clicked() {
                    // Get the currently hovered square
                    let mut currently_hovered = None;

                    // Loop over pieces

                    for y in 0..8 {
                        for x in 0..8 {
                            let square_id = format!("board_square_{y}x{x}");

                            // TODO: error handleing
                            let element2 = self.ui.try_get_element(square_id.clone()).unwrap();

                            // TODO: error handleing
                            let button2 = element2
                                .try_inner_mut::<crate::ui::element::Button>()
                                .unwrap();

                            if button2.get_state().hovered() {
                                currently_hovered = Some(square_id);
                                break;
                            }
                        }
                    }

                    if let Some(id) = currently_hovered {
                        let (x2, y2) = get_pos_from_id(&id);
                        let delta = (x2 - x, y2 - y);
                        info!("{delta:?}");
                    }
                }
            } else {
                for y in 0..8 {
                    for x in 0..8 {
                        let square_id = format!("board_square_{y}x{x}");

                        // Explicit unwrap is better than implicit
                        let element = self.ui.try_get_element(square_id.clone()).unwrap();

                        let element = element.inner_mut::<crate::ui::element::Button>();

                        if element.clicked_this_frame() {
                            self.current_drag = Some(square_id);
                        }
                    }
                }
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

    for i in 0..8 {
        for j in 0..8 {
            let position = shared::chess::Position::from_index(i, j).unwrap();
            if let Some((color, piece)) = board.read(position) {
                let sprite_id = SpriteId::ChessPiece(color, piece);

                let button = ui.get_element(format!("board_square_{i}x{j}"));

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

            let el = Element::new_button(
                format!("board_square_{i}x{}", 7 - j as u8), // This is nessesary due to the direction of the board vs the dierction of the nested loop
                square_center,
                square_size.clone(),
                if (i + j) as i32 % 2 == 0 {
                    style1
                } else {
                    style2
                },
            );

            ui.add_element(el, BOARD_UI_GROUP);
        }
    }
}
