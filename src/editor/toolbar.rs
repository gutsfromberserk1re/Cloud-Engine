use eframe::egui;
use crate::app::{KumoApp, Primitive, Tool};
use crate::theme;

pub fn top_bar(ctx: &egui::Context, app: &mut KumoApp) {
    egui::TopBottomPanel::top("menu_bar").exact_height(29.0).show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            for item in ["File", "Edit", "View", "Project", "Tools", "Window", "Help"] { ui.menu_button(item, |_ui| {}); }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(egui::RichText::new("KUMO ENGINE  ·  DEV").color(theme::MUTED).small());
            });
        });
    });

    egui::TopBottomPanel::top("toolbar").exact_height(48.0).show(ctx, |ui| {
        ui.horizontal_centered(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;
            for (label, tool, hint) in [("⌖", Tool::Select, "Select"), ("✥", Tool::Move, "Move"), ("⟳", Tool::Rotate, "Rotate"), ("⤢", Tool::Scale, "Scale")] {
                let selected = app.active_tool == tool;
                let button = egui::Button::new(egui::RichText::new(label).size(18.0).color(if selected { theme::PHOSPHOR } else { theme::TEXT }))
                    .min_size(egui::vec2(34.0, 32.0)).fill(if selected { theme::SELECTED } else { egui::Color32::TRANSPARENT })
                    .stroke(egui::Stroke::new(if selected { 1.0_f32 } else { 0.0_f32 }, theme::BORDER));
                if ui.add(button).on_hover_text(hint).clicked() { app.active_tool = tool; app.status = format!("{hint} tool selected"); }
            }

            ui.separator();
            // Main action inserts a cube immediately; the adjacent chevron selects another primitive.
            if ui.add(egui::Button::new(egui::RichText::new("＋  Insert Part").color(theme::PHOSPHOR)).min_size(egui::vec2(112.0, 31.0)).fill(theme::SELECTED)).on_hover_text("Insert a cube into the scene").clicked() {
                app.insert_part(Primitive::Cube);
            }
            ui.menu_button("∨", |ui| {
                ui.set_min_width(155.0);
                ui.label(egui::RichText::new("INSERT PRIMITIVE").small().color(theme::MUTED));
                ui.separator();
                for (primitive, label) in [
                    (Primitive::Cube, "▣  Cube"), (Primitive::Sphere, "●  Sphere"),
                    (Primitive::Cylinder, "▤  Cylinder"), (Primitive::Cone, "▲  Cone"),
                    (Primitive::Plane, "▱  Plane"), (Primitive::Triangle, "△  Triangle"),
                ] {
                    if ui.button(label).clicked() { app.insert_part(primitive); ui.close_menu(); }
                }
            });
            ui.separator();
            if ui.button("↶").on_hover_text("Undo (not implemented yet)").clicked() {}
            if ui.button("↷").on_hover_text("Redo (not implemented yet)").clicked() {}

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("■").on_hover_text("Stop").clicked() { app.playing = false; app.paused = false; }
                if ui.button(if app.paused { "▶" } else { "Ⅱ" }).on_hover_text("Pause / Resume").clicked() && app.playing { app.paused = !app.paused; }
                if ui.add(egui::Button::new(egui::RichText::new("▶").color(theme::PHOSPHOR)).fill(theme::SELECTED)).on_hover_text("Play").clicked() { app.playing = true; app.paused = false; }
                ui.separator();
                if ui.small_button("▤").on_hover_text("Toggle bottom panel").clicked() { app.show_bottom = !app.show_bottom; }
                if ui.small_button("▥").on_hover_text("Toggle Inspector").clicked() { app.show_inspector = !app.show_inspector; }
                if ui.small_button("☰").on_hover_text("Toggle Explorer").clicked() { app.show_explorer = !app.show_explorer; }
            });
        });
    });
}
