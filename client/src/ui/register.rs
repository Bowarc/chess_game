pub fn register_ui_elements(ui: &mut super::UiManager){
    let size = shared::maths::Point::new(10., 10.);
    
    let spacing = 10.;

    for i in 0..10{
        let i = i as f64;
        for j in 0..10{
            let j = j as f64;

            let centering = (
                super::value::MagicValue::ScreenSizeW / 2. - 10./2. * (size.x + spacing) ,
                super::value::MagicValue::ScreenSizeH / 2. - 10./2. * (size.y + spacing)
            );

            let pos = super::element::ElementPosition::new_value((
                centering.0 + super::Value::from(size.x * i + spacing * i) ,
                centering.1 + super::Value::from(size.y * j + spacing * j),
            ));

            let el = super::element::Element::new(
                super::element::ElementType::new_button(), 
                pos, 
                (size.x, size.y)
            );
            ui.add_element(el);
        }
    }
}