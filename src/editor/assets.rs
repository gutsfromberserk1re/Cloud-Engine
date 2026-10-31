use eframe::egui;

pub fn show(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label("⌕");
        ui.add(egui::TextEdit::singleline(&mut String::new()).hint_text("Search project assets...").desired_width(220.0));
        ui.separator();
        for item in ["All", "Models", "Textures", "Audio", "Materials"] {
            ui.selectable_label(item == "All", item);
        }
    });
    ui.add_space(8.0);
    ui.horizontal_wrapped(|ui| {
        for (icon, name) in [("◧", "Meshes"), ("▦", "Textures"), ("♫", "Audio"), ("◉", "Materials")] {
            ui.group(|ui| {
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new(icon).size(25.0));
                    ui.label(name);
                });
            });
        }
    });
}
