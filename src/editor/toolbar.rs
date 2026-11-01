use eframe::egui;
use crate::app::{KumoApp, Primitive, Tool};
use crate::theme;

pub fn top_bar(ctx: &egui::Context, app: &mut KumoApp) {
    egui::TopBottomPanel::top("menu_bar").exact_height(29.0).show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("New Scene     Ctrl+N").clicked() { app.new_scene(); ui.close_menu(); }
                if ui.button("Save Scene    Ctrl+S").clicked() { app.save_scene(); ui.close_menu(); }
                if ui.button("Load Scene").clicked() { app.load_scene(); ui.close_menu(); }
                ui.separator();
                if ui.button("Exit").clicked() { ctx.send_viewport_cmd(egui::ViewportCommand::Close); ui.close_menu(); }
            });
            ui.menu_button("Edit", |ui| {
                if ui.button("Duplicate       Ctrl+D").clicked() { app.duplicate_selection(); ui.close_menu(); }
                if ui.button("Delete          Delete").clicked() { app.delete_selection(); ui.close_menu(); }
                if ui.button("Focus Selection F").clicked() { app.focus_selection(); ui.close_menu(); }
                if ui.button("Clear Selection Esc").clicked() { app.selection = crate::app::Selection::None; ui.close_menu(); }
            });
            ui.menu_button("View", |ui| {
                ui.checkbox(&mut app.show_grid, "Show grid");
                ui.checkbox(&mut app.show_gizmos, "Show transform gizmos");
                ui.separator();
                ui.checkbox(&mut app.show_explorer, "Explorer");
                ui.checkbox(&mut app.show_inspector, "Inspector");
                ui.checkbox(&mut app.show_console, "Console");
            });
            ui.menu_button("Window", |ui| {
                if ui.button("Toggle Explorer").clicked() { app.show_explorer = !app.show_explorer; ui.close_menu(); }
                if ui.button("Toggle Inspector").clicked() { app.show_inspector = !app.show_inspector; ui.close_menu(); }
                if ui.button("Toggle Console").clicked() { app.show_console = !app.show_console; ui.close_menu(); }
            });
            ui.menu_button("Help", |ui| {
                ui.label(egui::RichText::new("VIEWPORT CONTROLS").small().color(theme::PHOSPHOR));
                ui.label("Right mouse + drag  ·  Look around");
                ui.label("W A S D              ·  Move camera");
                ui.label("Q / E                ·  Down / up");
                ui.label("Arrow keys           ·  Move camera");
                ui.label("Mouse wheel          ·  Dolly");
                ui.separator();
                ui.label(egui::RichText::new("TOOLS").small().color(theme::PHOSPHOR));
                ui.label("1 Select  ·  2 Move  ·  3 Rotate  ·  4 Resize");
                ui.label("F Focus  ·  Delete Remove  ·  Ctrl+D Duplicate");
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(egui::RichText::new("KUMO ENGINE  ·  SCENE EDITOR").color(theme::MUTED).small());
            });
        });
    });

    egui::TopBottomPanel::top("toolbar").exact_height(48.0).show(ctx, |ui| {
        ui.horizontal_centered(|ui| {
            ui.spacing_mut().item_spacing.x = 5.0;
            for (tool, key) in [(Tool::Select,"1"),(Tool::Move,"2"),(Tool::Rotate,"3"),(Tool::Scale,"4")] {
                let (rect, response) = ui.allocate_exact_size(egui::vec2(36.0, 34.0), egui::Sense::click());
                let selected = app.active_tool == tool;
                let fill = if selected { theme::SELECTED } else if response.hovered() { theme::PANEL_ALT } else { egui::Color32::TRANSPARENT };
                ui.painter().rect_filled(rect, 4.0, fill);
                if selected { ui.painter().rect_stroke(rect, 4.0, egui::Stroke::new(1.0_f32, theme::BORDER)); }
                draw_tool_icon(ui.painter(), rect.shrink(7.0), tool, if selected { theme::PHOSPHOR } else { theme::TEXT });
                let response = response.on_hover_text(format!("{} tool ({key})", tool.label()));
                if response.clicked() { app.set_tool(tool); }
            }
            ui.separator();
            if ui.add(egui::Button::new(egui::RichText::new("＋  Insert").color(theme::PHOSPHOR)).min_size(egui::vec2(94.0, 31.0)).fill(theme::SELECTED)).on_hover_text("Insert a primitive object").clicked() {
                app.insert_part(Primitive::Cube);
            }
            ui.menu_button("▾", |ui| {
                ui.set_min_width(150.0);
                ui.label(egui::RichText::new("INSERT GEOMETRY").small().color(theme::MUTED));
                ui.separator();
                for primitive in Primitive::all() {
                    if ui.button(format!("＋  {}", primitive.label())).clicked() { app.insert_part(primitive); ui.close_menu(); }
                }
            });
            ui.separator();
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.small_button("▤").on_hover_text("Toggle Console").clicked() { app.show_console = !app.show_console; }
                if ui.small_button("◫").on_hover_text("Toggle Inspector").clicked() { app.show_inspector = !app.show_inspector; }
                if ui.small_button("▥").on_hover_text("Toggle Explorer").clicked() { app.show_explorer = !app.show_explorer; }
            });
        });
    });
}

fn draw_tool_icon(painter: &egui::Painter, rect: egui::Rect, tool: Tool, color: egui::Color32) {
    let center = rect.center();
    let stroke = egui::Stroke::new(1.7_f32, color);
    match tool {
        Tool::Select => {
            let pts = vec![rect.left_top(), egui::pos2(rect.left()+2.0, rect.bottom()-1.0), egui::pos2(center.x, center.y+2.0), egui::pos2(center.x+3.0, rect.bottom()), egui::pos2(center.x+6.0, center.y+1.0)];
            painter.add(egui::Shape::convex_polygon(pts, color, egui::Stroke::NONE));
        }
        Tool::Move => {
            painter.line_segment([egui::pos2(center.x,rect.top()),egui::pos2(center.x,rect.bottom())],stroke);
            painter.line_segment([egui::pos2(rect.left(),center.y),egui::pos2(rect.right(),center.y)],stroke);
            painter.line_segment([egui::pos2(center.x,rect.top()),egui::pos2(center.x-3.0,rect.top()+4.0)],stroke);
            painter.line_segment([egui::pos2(center.x,rect.top()),egui::pos2(center.x+3.0,rect.top()+4.0)],stroke);
            painter.line_segment([egui::pos2(rect.right(),center.y),egui::pos2(rect.right()-4.0,center.y-3.0)],stroke);
            painter.line_segment([egui::pos2(rect.right(),center.y),egui::pos2(rect.right()-4.0,center.y+3.0)],stroke);
            painter.circle_filled(center,2.1,color);
        }
        Tool::Rotate => {
            let points: Vec<egui::Pos2> = (0..28).map(|i| {
                let a = (i as f32 / 28.0_f32) * std::f32::consts::TAU * 0.82_f32 - 0.35_f32;
                center + egui::vec2(a.cos()*8.0,a.sin()*8.0)
            }).collect();
            painter.add(egui::Shape::line(points, stroke));
            let tip = center + egui::vec2(8.0,-5.5);
            painter.line_segment([tip,tip+egui::vec2(-1.0,4.0)],stroke);
            painter.line_segment([tip,tip+egui::vec2(-4.0,1.0)],stroke);
        }
        Tool::Scale => {
            let square = rect.shrink(1.5);
            painter.rect_stroke(square,1.0,stroke);
            painter.line_segment([square.left_top(),square.left_top()+egui::vec2(4.0,0.0)],stroke);
            painter.line_segment([square.left_top(),square.left_top()+egui::vec2(0.0,4.0)],stroke);
            painter.line_segment([square.right_bottom(),square.right_bottom()-egui::vec2(4.0,0.0)],stroke);
            painter.line_segment([square.right_bottom(),square.right_bottom()-egui::vec2(0.0,4.0)],stroke);
        }
    }
}
