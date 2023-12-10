global_ui.add_element(
    ui::element::Element::new_text(
        "league_ui_test",
        (ui::Anchor::TopCenter, (0., 100.)),
        40.,
        ui::Style::new(
            render::Color::random_rgb(),
            None,
            Some(ui::style::BorderStyle::new(render::Color::random_rgb(), 1.)),
        ),
        vec![
            ui::element::TextBit::new_text(
                "15 ",
                Some(render::Color::from_rgb(186, 80, 40)),
            ),
            ui::element::TextBit::new_text(
                "(+1.1",
                Some(render::Color::from_rgb(215, 148, 95)),
            ),
            ui::element::TextBit::new_img(
                assets::sprite::SpriteId::AttackDamage,
                Some(render::Color::from_rgb(215, 148, 95)),
            ),
            ui::element::TextBit::new_text(
                ")",
                Some(render::Color::from_rgb(215, 148, 95)),
            ),
            ui::element::TextBit::new_text(
                " (+0.4",
                Some(render::Color::from_rgb(105, 255, 249)),
            ),
            ui::element::TextBit::new_img(
                assets::sprite::SpriteId::AbilityPower,
                Some(render::Color::from_rgb(105, 255, 249)),
            ),
            ui::element::TextBit::new_text(
                ")",
                Some(render::Color::from_rgb(105, 255, 249)),
            ),
        ],
    ),
    "",
);