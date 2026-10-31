use eframe::egui;
use crate::{app::KumoApp, theme};

pub fn show(ui: &mut egui::Ui, app: &mut KumoApp) {
    ui.horizontal(|ui| {
        ui.strong("⌘  EXPLORER");
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| { if ui.small_button("×").clicked() { app.show_explorer = false; } });
    });
    ui.separator();
    ui.label(egui::RichText::new("⌕  Scene hierarchy").color(theme::MUTED).small());
    ui.add_space(5.0);
    egui::ScrollArea::vertical().show(ui, |ui| {
        egui::CollapsingHeader::new("◈  Workspace").default_open(true).show(ui, |ui| {
            object_row(ui, app, "Camera", "◉");
            object_row(ui, app, "Lighting", "☼");
            for object in app.objects.clone() { object_row(ui, app, &object.name, object.primitive.icon()); }
            object_row(ui, app, "SpawnPoint", "✣");
        });
        for (name, icon) in [("Environment", "◈"), ("Materials", "◐"), ("Audio", "♫"), ("Scripts", "⌘"), ("UI", "▦")] {
            egui::CollapsingHeader::new(format!("{icon}  {name}")).default_open(false).show(ui, |_ui| {});
        }
    });
}

fn object_row(ui: &mut egui::Ui, app: &mut KumoApp, name: &str, icon: &str) {
    let selected = app.selected_object == name;
    let response = ui.selectable_label(selected, format!("{icon}  {name}"));
    if response.clicked() { app.selected_object = name.to_owned(); app.status = format!("Selected {name}"); }
    if selected { ui.painter().rect_filled(response.rect, 2.0, theme::SELECTED.gamma_multiply(0.35)); }
}
