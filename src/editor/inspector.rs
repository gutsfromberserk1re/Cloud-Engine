use eframe::egui;
use crate::{app::KumoApp, theme};

pub fn show(ui: &mut egui::Ui, app: &mut KumoApp) {
    ui.horizontal(|ui| {
        ui.strong("◫  INSPECTOR");
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.small_button("×").clicked() { app.show_inspector = false; }
        });
    });
    ui.separator();
    ui.label(egui::RichText::new(&app.selected_object).color(theme::PHOSPHOR).strong());
    if let Some(shape) = app.selected_primitive() { property(ui, "Primitive", shape.label()); }
    egui::ScrollArea::vertical().show(ui, |ui| {
        egui::CollapsingHeader::new("Transform").default_open(true).show(ui, |ui| {
            vector_row(ui, "Position", ["0.00", "0.00", "0.00"]);
            vector_row(ui, "Rotation", ["0.00", "0.00", "0.00"]);
            vector_row(ui, "Scale", ["1.00", "1.00", "1.00"]);
        });
        egui::CollapsingHeader::new("Appearance").default_open(true).show(ui, |ui| {
            property(ui, "Material", "Default");
            property(ui, "Colour", "240, 244, 250");
            property(ui, "Metallic", "0.00");
            property(ui, "Roughness", "0.65");
        });
        egui::CollapsingHeader::new("Rendering").default_open(false).show(ui, |ui| {
            property(ui, "Cast Shadows", "On");
            property(ui, "Visible", "True");
        });
        egui::CollapsingHeader::new("Physics").default_open(false).show(ui, |ui| {
            property(ui, "Collider", "Box");
            property(ui, "Mass", "1.00");
        });
        ui.add_space(10.0);
        ui.add_sized([ui.available_width(), 30.0], egui::Button::new("＋  Add Component"));
    });
}

fn vector_row(ui: &mut egui::Ui, label: &str, values: [&str; 3]) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(label).color(theme::MUTED));
        for value in values {
            ui.add_sized([48.0, 22.0], egui::Label::new(egui::RichText::new(value).monospace()));
        }
    });
}
fn property(ui: &mut egui::Ui, key: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(key).color(theme::MUTED));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(value);
        });
    });
}
