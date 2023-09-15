pub fn draw_credits(ui: &mut ggegui::egui::Ui) {
    ui.horizontal(|ui| {
        ui.label("Thanks to");

        ui.hyperlink_to("GGEZ", "https://github.com/ggez/ggez");

        ui.label("and its great comunity, it's the framework used to make this game")
    });
    ui.separator();
    ui.horizontal(|ui| {
        ui.label("Thanks to Abstraction for his kindness and amazing musics, he can be found at");
        ui.hyperlink("https://www.abstractionmusic.com/")
    });
    ui.separator();
}
