const BOARD_UI_GROUP: &str = "board";
const BOARD_SPRITE_UI_GROUP: &str = "board_sprite";

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


        if self.current_game.changed(){
            if let shared::game::State::Playing { board } = self.current_game.inner().unwrap().state(){
                if self.ui.get_group(BOARD_UI_GROUP).is_none() {
                    create_board(&mut self.ui);
                    create_board_pieces(&mut self.ui, board)
                }
            }else{
                return super::State::from_shared_state(
                    self.client,
                    self.current_game.inner().cloned().unwrap(),
                );
            }
        }

        self.ui.update(ggctx);

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

fn create_board_pieces(ui: &mut crate::ui::UiManager, board: &shared::chess::Board) {
    use crate::{
        assets::sprite::SpriteId,
        ui::{element::Element, element::TextBit, Style},
    };

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
                    vec![
                        TextBit::new_img(sprite_id, None)
                    ],
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
