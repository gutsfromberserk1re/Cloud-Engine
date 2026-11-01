use eframe::egui;
use crate::app::{KumoApp, Selection};
use crate::theme;

pub fn show(ui: &mut egui::Ui, app: &mut KumoApp) {
    ui.horizontal(|ui| {
        ui.strong("▤  EXPLORER");
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.small_button("×").on_hover_text("Hide Explorer").clicked() { app.show_explorer = false; }
            if ui.small_button("＋").on_hover_text("Create a folder").clicked() { app.create_folder(None); }
        });
    });
    ui.separator();
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("SCENE CONTENTS").color(theme::MUTED).small());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(egui::RichText::new(format!("{}", app.objects.len() + app.folders.len())).color(theme::MUTED).small());
        });
    });

    let available = ui.available_rect_before_wrap();
    let background_response = ui.interact(available, ui.id().with("explorer_background"), egui::Sense::click());
    background_response.context_menu(|ui| {
        if ui.button("＋  New folder").clicked() { app.create_folder(None); ui.close_menu(); }
    });

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(3.0);
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("◈").color(theme::PHOSPHOR));
            ui.label(egui::RichText::new("Scene").strong());
            ui.label(egui::RichText::new("· root").small().color(theme::MUTED));
        });
        let roots: Vec<u32> = app.folders.iter().filter(|f| f.parent.is_none()).map(|f| f.id).collect();
        for id in roots { folder_row(ui, app, id, 1); }
        let root_objects: Vec<u32> = app.objects.iter().filter(|o| o.parent.is_none()).map(|o| o.id).collect();
        for id in root_objects { object_row(ui, app, id, 1); }
        if app.folders.is_empty() && app.objects.is_empty() {
            ui.add_space(10.0);
            ui.label(egui::RichText::new("Empty scene. Use + to create folders, or Insert to add geometry.").small().color(theme::MUTED));
        }
        ui.add_space(12.0);
        ui.label(egui::RichText::new("RIGHT-CLICK TO ORGANIZE").small().color(theme::MUTED));
    });
}

fn folder_row(ui: &mut egui::Ui, app: &mut KumoApp, id: u32, depth: usize) {
    let Some(folder) = app.folders.iter().find(|f| f.id == id).cloned() else { return; };
    let expanded = app.expanded_folders.contains(&id);
    ui.horizontal(|ui| {
        ui.add_space((depth as f32) * 12.0);
        let arrow = if expanded { "▾" } else { "▸" };
        if ui.small_button(arrow).clicked() {
            if expanded { app.expanded_folders.retain(|v| *v != id); } else { app.expanded_folders.push(id); }
        }
        let is_selected = app.selection == Selection::Folder(id);
        if app.renaming_folder == Some(id) {
            let response = ui.add(egui::TextEdit::singleline(&mut app.rename_buffer).desired_width(125.0));
            let enter = ui.input(|i| i.key_pressed(egui::Key::Enter));
            if response.lost_focus() || enter {
                let name = app.rename_buffer.trim().to_owned();
                if !name.is_empty() { if let Some(f) = app.folders.iter_mut().find(|f| f.id == id) { f.name = name; } }
                app.renaming_folder = None;
            }
            response.request_focus();
        } else {
            let response = ui.selectable_label(is_selected, format!("📁  {}", folder.name));
            if response.clicked() { app.selection = Selection::Folder(id); app.status = format!("Selected folder {}", folder.name); }
            response.context_menu(|ui| {
                if ui.button("＋  Add subfolder").clicked() { app.create_folder(Some(id)); ui.close_menu(); }
                if ui.button("Rename").clicked() { app.renaming_folder = Some(id); app.rename_buffer = folder.name.clone(); ui.close_menu(); }
                if ui.button("Delete folder and contents").clicked() { app.selection = Selection::Folder(id); app.delete_selection(); ui.close_menu(); }
            });
        }
    });
    if expanded {
        let children: Vec<u32> = app.folders.iter().filter(|f| f.parent == Some(id)).map(|f| f.id).collect();
        for child in children { folder_row(ui, app, child, depth + 1); }
        let objects: Vec<u32> = app.objects.iter().filter(|o| o.parent == Some(id)).map(|o| o.id).collect();
        for object_id in objects { object_row(ui, app, object_id, depth + 1); }
    }
}

fn object_row(ui: &mut egui::Ui, app: &mut KumoApp, id: u32, depth: usize) {
    let Some(object) = app.objects.iter().find(|o| o.id == id).cloned() else { return; };
    ui.horizontal(|ui| {
        ui.add_space((depth as f32) * 12.0 + 16.0);
        let selected = app.selection == Selection::Object(id);
        let response = ui.selectable_label(selected, format!("◇  {}", object.name));
        if response.clicked() { app.selection = Selection::Object(id); app.status = format!("Selected {}", object.name); }
        response.context_menu(|ui| {
            if ui.button("Duplicate").clicked() { app.selection = Selection::Object(id); app.duplicate_selection(); ui.close_menu(); }
            ui.menu_button("Move to folder", |ui| {
                if ui.button("Scene root").clicked() {
                    if let Some(object) = app.objects.iter_mut().find(|o| o.id == id) { object.parent = None; }
                    ui.close_menu();
                }
                let destinations: Vec<(u32,String)> = app.folders.iter().map(|f|(f.id,f.name.clone())).collect();
                for (folder_id, name) in destinations {
                    if ui.button(format!("📁  {name}")).clicked() {
                        if let Some(object) = app.objects.iter_mut().find(|o| o.id == id) { object.parent = Some(folder_id); }
                        ui.close_menu();
                    }
                }
            });
            if ui.button("Delete").clicked() { app.selection = Selection::Object(id); app.delete_selection(); ui.close_menu(); }
        });
    });
}
