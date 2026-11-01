use eframe::egui;
use crate::app::{KumoApp, Selection};
use crate::theme;

pub fn show(ui: &mut egui::Ui, app: &mut KumoApp) {
    ui.horizontal(|ui| {
        ui.strong("◫  INSPECTOR");
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.small_button("×").on_hover_text("Hide Inspector").clicked() { app.show_inspector = false; }
        });
    });
    ui.separator();
    match app.selection {
        Selection::Object(id) => object_inspector(ui, app, id),
        Selection::Folder(id) => folder_inspector(ui, app, id),
        Selection::None => {
            ui.add_space(8.0);
            ui.label(egui::RichText::new("Nothing selected").color(theme::MUTED));
            ui.label(egui::RichText::new("Select an object or folder in the scene.").small().color(theme::MUTED));
        }
    }
}

fn object_inspector(ui: &mut egui::Ui, app: &mut KumoApp, id: u32) {
    let Some(index) = app.objects.iter().position(|o| o.id == id) else { app.selection = Selection::None; return; };
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("OBJECT").small().color(theme::MUTED));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.small_button("⌫").on_hover_text("Delete selected object").clicked() { app.delete_selection(); }
            if ui.small_button("⧉").on_hover_text("Duplicate selected object").clicked() { app.duplicate_selection(); }
        });
    });
    let object = &mut app.objects[index];
    ui.add(egui::TextEdit::singleline(&mut object.name).desired_width(ui.available_width()).hint_text("Object name"));
    ui.label(egui::RichText::new(object.primitive.label()).color(theme::PHOSPHOR).small());
    ui.add_space(8.0);

    egui::CollapsingHeader::new("Transform").default_open(true).show(ui, |ui| {
        vector_edit(ui, "Position", &mut object.position, 0.1);
        vector_edit(ui, "Rotation", &mut object.rotation, 1.0);
        vector_edit(ui, "Scale", &mut object.scale, 0.05);
        object.scale.x = object.scale.x.max(0.01);
        object.scale.y = object.scale.y.max(0.01);
        object.scale.z = object.scale.z.max(0.01);
    });
    egui::CollapsingHeader::new("Hierarchy").default_open(true).show(ui, |ui| {
        let mut parent = object.parent;
        egui::ComboBox::from_id_source(("object_parent", id)).selected_text(
            parent.and_then(|p| app.folders.iter().find(|f| f.id == p).map(|f| f.name.clone())).unwrap_or_else(|| "Scene root".to_owned())
        ).show_ui(ui, |ui| {
            ui.selectable_value(&mut parent, None, "Scene root");
            for folder in &app.folders { ui.selectable_value(&mut parent, Some(folder.id), folder.name.clone()); }
        });
        // Keep folder parenting acyclic: a folder can contain objects, never folders' ancestors.
        if parent.map(|p| app.folders.iter().any(|f| f.id == p)).unwrap_or(true) { object.parent = parent; }
    });
}

fn folder_inspector(ui: &mut egui::Ui, app: &mut KumoApp, id: u32) {
    let Some(index) = app.folders.iter().position(|f| f.id == id) else { app.selection = Selection::None; return; };
    ui.label(egui::RichText::new("FOLDER").small().color(theme::MUTED));
    ui.add(egui::TextEdit::singleline(&mut app.folders[index].name).desired_width(ui.available_width()));
    ui.add_space(8.0);
    let child_folders = app.folders.iter().filter(|f| f.parent == Some(id)).count();
    let child_objects = app.objects.iter().filter(|o| o.parent == Some(id)).count();
    ui.label(format!("{child_folders} folders · {child_objects} objects"));
    ui.add_space(8.0);
    if ui.button("＋  Add subfolder").clicked() { app.create_folder(Some(id)); }
    if ui.button("Delete folder and contents").clicked() { app.delete_selection(); }
}

fn vector_edit(ui: &mut egui::Ui, label: &str, value: &mut crate::app::Vec3, speed: f32) {
    ui.horizontal(|ui| {
        ui.add_sized([62.0, 20.0], egui::Label::new(egui::RichText::new(label).color(theme::MUTED)));
        ui.add(egui::DragValue::new(&mut value.x).speed(speed).prefix("X ").max_decimals(2));
        ui.add(egui::DragValue::new(&mut value.y).speed(speed).prefix("Y ").max_decimals(2));
        ui.add(egui::DragValue::new(&mut value.z).speed(speed).prefix("Z ").max_decimals(2));
    });
}
