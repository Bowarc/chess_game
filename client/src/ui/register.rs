pub fn register_ui_elements(ui: &mut super::UiManager) {
    let build_style = |main_color: &str| -> super::style::Bundle {
        super::style::Bundle::new(
            super::Style::new(crate::render::Color::from_hex(main_color), None, None),
            Some(super::Style::new(
                crate::render::Color::from_hex(&format!("{main_color}aa")),
                None,
                Some(super::style::BorderStyle::new(
                    crate::render::Color::from_hex("#000000"),
                    5.,
                )),
            )),
            Some(super::Style::new(
                crate::render::Color::from_hex(&format!("{main_color}55")),
                None,
                Some(super::style::BorderStyle::new(
                    crate::render::Color::from_hex("#000000"),
                    5.,
                )),
            )),
        )
    };

    let board_size = 600.;

    let nbr_of_square: u8 = 8;

    let size = shared::maths::Point::new(board_size / 8., board_size / 8.);

    let spacing = 0.;

    let style1 = build_style("#b88b4a");

    let style2 = build_style("#e3c16f");

    for i in 0..nbr_of_square {
        let i = i as f64;
        for j in 0..nbr_of_square {
            let j = j as f64;

            let centering = (
                super::value::MagicValue::ScreenSizeW * 0.5
                    - nbr_of_square as f64 * 0.5 * (size.x + spacing)
                    + size.x * 0.5,
                super::value::MagicValue::ScreenSizeH * 0.5
                    - nbr_of_square as f64 * 0.5 * (size.y + spacing)
                    + size.y * 0.5,
            );

            let pos = super::Position::new_value((
                centering.0 + super::Value::from(size.x * i + spacing * i),
                centering.1 + super::Value::from(size.y * j + spacing * j),
            ));

            let el = super::element::Element::new_button(
                format!("board square {i}x{j}"),
                pos,
                (size.x, size.y),
                if (i + j) as i32 % 2 == 0 {
                    style1
                } else {
                    style2
                },
            );
            ui.add_element(el);
        }
    }
}
