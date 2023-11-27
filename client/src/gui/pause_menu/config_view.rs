use {crate::config, crate::gui::pause_menu, ggegui::egui};

pub fn draw_config_menu(
    ctx: &mut ggez::Context,
    ui: &mut egui::Ui,
    cfg: &mut config::Config,
    overtime_values: &mut pause_menu::OverTimeValues,
) {
    // ui.label("Rows enter from the bottom, we want the scroll handle to start and stay at bottom unless moved");

    // ui.add_space(4.0);

    let text_style = egui::TextStyle::Small;
    let row_height = ui.text_style_height(&text_style);
    // ui.separator();

    egui::ScrollArea::vertical()
        .stick_to_bottom(false)
        .max_height(pause_menu::WINDOW_SIZE.1)
        .max_width(pause_menu::WINDOW_SIZE.0)
        .min_scrolled_height(row_height)
        .show(ui, |ui| {
            ui.label("Config:");
            draw_user_config_menu(ctx, ui, &mut cfg.user, overtime_values);
            ui.separator();
            draw_window_config_menu(ctx, ui, &mut cfg.window, overtime_values);
            ui.separator();
            draw_render_config_menu(ctx, ui, &mut cfg.render, overtime_values);
            ui.separator();
            draw_audio_config_menu(ctx, ui, &mut cfg.audio, overtime_values);
            ui.separator();
            draw_optimisation_config_menu(ctx, ui, &mut cfg.optimisation, overtime_values);
            ui.separator()
        });
    // ui.separator();
}

fn draw_user_config_menu(
    _ctx: &mut ggez::Context,
    ui: &mut egui::Ui,
    _cfg: &mut config::UserConfig,
    _overtime_values: &mut pause_menu::OverTimeValues,
) {
    ui.label("User Config");
}

fn draw_window_config_menu(
    ctx: &mut ggez::Context,
    ui: &mut egui::Ui,
    cfg: &mut config::WindowConfig,
    overtime_values: &mut pause_menu::OverTimeValues,
) {
    ui.label("Window Config");

    // v_sync
    ui.horizontal(|ui| {
        ui.label("Vertical Sync (reboot required)");
        if ui.button(format!("{:?}", cfg.v_sync)).clicked() {
            cfg.v_sync = !cfg.v_sync;
        }
    });
    // size
    ui.horizontal(|ui| {
        ui.label("Size");

        let graphic_context_size_slider_index = overtime_values.graphic_context_size_slider_index;
        ui.add(
            egui::Slider::new(
                &mut overtime_values.graphic_context_size_slider_index,
                0.0..=overtime_values.supported_resolutions.len() as f32 - 1.0,
            )
            .step_by(1.)
            .show_value(false)
            .text(format!(
                "{}x{}",
                overtime_values.supported_resolutions[graphic_context_size_slider_index as usize]
                    .width,
                overtime_values.supported_resolutions[graphic_context_size_slider_index as usize]
                    .height
            )),
        );
        if ui.button("Update").clicked() {
            let x = overtime_values.supported_resolutions
                [graphic_context_size_slider_index as usize]
                .width;
            let y = overtime_values.supported_resolutions
                [graphic_context_size_slider_index as usize]
                .height;

            cfg.size = (x as i32, y as i32);
            ctx.gfx.set_mode((*cfg).into()).unwrap();

            debug!("new window config: {:#?}", cfg);
        }
    });
    // fullscreen_type
    ui.horizontal(|ui| {
        egui::ComboBox::from_label("Fullcreen type")
            .selected_text(format!("{:?}", overtime_values.selected_fullscreen_type))
            .show_ui(ui, |ui| {
                for screen_type in [
                    ggez::conf::FullscreenType::Windowed,
                    ggez::conf::FullscreenType::True,
                    ggez::conf::FullscreenType::Desktop,
                ] {
                    ui.selectable_value(
                        &mut overtime_values.selected_fullscreen_type,
                        screen_type,
                        format!("{screen_type:?}"),
                    );
                }
            });
        if ui.button("Update").clicked() {
            cfg.fullscreen_type = overtime_values.selected_fullscreen_type;
            ctx.gfx.set_mode((*cfg).into()).unwrap();

            debug!("new window config: {:#?}", cfg);
        }
    });
    // samples
    ui.horizontal(|ui| ui.label("Samples are for now in WIP"));
    // SRGB
    ui.horizontal(|ui| ui.label("SRGB is for now in WIP"));
}

fn draw_render_config_menu(
    _ctx: &mut ggez::Context,
    ui: &mut egui::Ui,
    _cfg: &mut config::RenderConfig,
    _overtime_values: &mut pause_menu::OverTimeValues,
) {
    ui.label("Render Config");
}

fn draw_audio_config_menu(
    _ctx: &mut ggez::Context,
    ui: &mut egui::Ui,
    cfg: &mut config::AudioConfig,
    _overtime_values: &mut pause_menu::OverTimeValues,
) {
    ui.label("Audio Config");
    ui.horizontal(|ui| {
        ui.label("enabled");
        if ui.button(format!("{}", cfg.enabled)).clicked() {
            cfg.enabled = !cfg.enabled;
        }
    });

    ui.horizontal(|ui| {
        ui.add(
            egui::Slider::new(&mut cfg.global_vol, 0.0..=1.)
                .step_by(0.01)
                .show_value(true)
                .text("Global volume".to_string()),
        );
    });

    ui.horizontal(|ui| {
        ui.add(
            egui::Slider::new(&mut cfg.music_vol, 0.0..=1.)
                .step_by(0.01)
                .show_value(true)
                .text("Music volume".to_string()),
        );
    });

    ui.horizontal(|ui| {
        ui.add(
            egui::Slider::new(&mut cfg.gameplay_vol, 0.0..=1.)
                .step_by(0.01)
                .show_value(true)
                .text("Gameplay volume".to_string()),
        );
    });
}

fn draw_optimisation_config_menu(
    _ctx: &mut ggez::Context,
    ui: &mut egui::Ui,
    _cfg: &mut config::OptimisationConfig,
    _overtime_values: &mut pause_menu::OverTimeValues,
) {
    ui.label("Optimisation Config");
}
