use eframe::egui;

use crate::editor::{assets, explorer, inspector, toolbar, viewport};
use crate::theme;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tool { Select, Move, Rotate, Scale }

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BottomTab { Console, Assets }

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Primitive { Cube, Sphere, Cylinder, Cone, Plane, Triangle }

impl Primitive {
    pub fn label(self) -> &'static str {
        match self {
            Self::Cube => "Cube", Self::Sphere => "Sphere", Self::Cylinder => "Cylinder",
            Self::Cone => "Cone", Self::Plane => "Plane", Self::Triangle => "Triangle",
        }
    }
    pub fn icon(self) -> &'static str {
        match self {
            Self::Cube => "▣", Self::Sphere => "●", Self::Cylinder => "▤",
            Self::Cone => "▲", Self::Plane => "▱", Self::Triangle => "△",
        }
    }
}

#[derive(Clone)]
pub struct SceneObject {
    pub name: String,
    pub primitive: Primitive,
}

pub struct KumoApp {
    pub active_tool: Tool,
    pub playing: bool,
    pub paused: bool,
    pub selected_object: String,
    pub objects: Vec<SceneObject>,
    next_part_id: u32,
    pub bottom_tab: BottomTab,
    pub show_explorer: bool,
    pub show_inspector: bool,
    pub show_bottom: bool,
    pub status: String,
}

impl KumoApp {
    pub fn insert_part(&mut self, primitive: Primitive) {
        let name = format!("{}.{:03}", primitive.label(), self.next_part_id);
        self.next_part_id += 1;
        self.objects.push(SceneObject { name: name.clone(), primitive });
        self.selected_object = name.clone();
        self.status = format!("Inserted {}", name);
    }

    pub fn selected_primitive(&self) -> Option<Primitive> {
        self.objects.iter().find(|o| o.name == self.selected_object).map(|o| o.primitive)
    }
}

impl Default for KumoApp {
    fn default() -> Self {
        Self {
            active_tool: Tool::Move,
            playing: false,
            paused: false,
            selected_object: "Baseplate".to_owned(),
            objects: vec![SceneObject { name: "Baseplate".to_owned(), primitive: Primitive::Plane }],
            next_part_id: 1,
            bottom_tab: BottomTab::Console,
            show_explorer: true,
            show_inspector: true,
            show_bottom: true,
            status: "Ready".to_owned(),
        }
    }
}

impl eframe::App for KumoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        theme::apply_visuals(ctx);
        toolbar::top_bar(ctx, self);

        if self.show_explorer {
            egui::SidePanel::left("explorer_panel").resizable(true).default_width(225.0)
                .min_width(150.0).max_width(360.0).show(ctx, |ui| explorer::show(ui, self));
        }
        if self.show_inspector {
            egui::SidePanel::right("inspector_panel").resizable(true).default_width(285.0)
                .min_width(210.0).max_width(420.0).show(ctx, |ui| inspector::show(ui, self));
        }
        if self.show_bottom {
            egui::TopBottomPanel::bottom("bottom_panel").resizable(true).default_height(190.0)
                .min_height(90.0).max_height(360.0).show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut self.bottom_tab, BottomTab::Console, "⌘  Console");
                        ui.selectable_value(&mut self.bottom_tab, BottomTab::Assets, "▦  Assets");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.small_button("×").clicked() { self.show_bottom = false; }
                        });
                    });
                    ui.separator();
                    match self.bottom_tab {
                        BottomTab::Console => {
                            ui.monospace("[Kumo] Editor initialized");
                            ui.monospace("[Kumo] Primitive scene objects are editor-side placeholders for now");
                            ui.monospace("[Kumo] Ready for project");
                            ui.add_space(5.0);
                            ui.colored_label(theme::PHOSPHOR, "›");
                        }
                        BottomTab::Assets => assets::show(ui),
                    }
                });
        }
        egui::CentralPanel::default().show(ctx, |ui| viewport::show(ui, self));
        egui::TopBottomPanel::bottom("status_bar").exact_height(23.0).show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.label(egui::RichText::new(&self.status).color(theme::MUTED));
                ui.separator();
                ui.label(egui::RichText::new("OpenGL UI · 3D renderer pending").color(theme::MUTED));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new(format!("FPS: --   |   Objects: {}", self.objects.len())).color(theme::MUTED));
                });
            });
        });
        ctx.request_repaint();
    }
}
