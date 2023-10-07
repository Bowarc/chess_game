pub fn register_ui_elements(ui: &mut super::UiManager) {
    let size = shared::maths::Point::new(10., 10.);

    let spacing = 20.;

    let mut style_bundle = super::style::Bundle::default();
    *style_bundle.get_default_mut().get_color_mut() = crate::render::Color::from_rgb(0, 175, 150);
    *style_bundle
        .get_default_mut()
        .get_border_mut()
        .unwrap()
        .get_size_mut() = 0.5;

    let mut tbundle = super::style::Bundle::default();
    *tbundle.get_default_mut().get_color_mut() = crate::render::Color::WHITE;
    *tbundle
        .get_default_mut()
        .get_border_mut()
        .unwrap()
        .get_color_mut() = crate::render::Color::from_rgb(0, 255, 0);

    debug!("1: {style_bundle:?}\n2: {tbundle:?}");

    for i in 0..10 {
        let i = i as f64;
        for j in 0..10 {
            let j = j as f64;

            let centering = (
                super::value::MagicValue::ScreenSizeW / 2. - 10. / 2. * (size.x + spacing),
                super::value::MagicValue::ScreenSizeH / 2. - 10. / 2. * (size.y + spacing),
            );

            let pos = super::Position::new_value((
                centering.0 + super::Value::from(size.x * i + spacing * i),
                centering.1 + super::Value::from(size.y * j + spacing * j),
            ));

            let el = super::element::Element::new(
                super::element::ElementType::new_button(),
                pos,
                (size.x, size.y),
                if j as i32 % 2 == 0 {
                    tbundle
                } else {
                    style_bundle
                },
            );
            ui.add_element(el);
        }
    }
}
