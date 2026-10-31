use eframe::egui;
use crate::{app::{KumoApp, Primitive}, theme};

pub fn show(ui: &mut egui::Ui, app: &mut KumoApp) {
    ui.horizontal(|ui| {
        ui.selectable_label(true, "◈  Scene");
        ui.selectable_label(false, "▷  Game");
        ui.separator();
        ui.label(egui::RichText::new("Perspective  ·  Shaded").color(theme::MUTED).small());
    });
    ui.separator();
    let (rect, _response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 2.0, egui::Color32::from_rgb(35, 39, 45));
    let horizon = rect.top() + rect.height() * 0.48;
    let floor = egui::Rect::from_min_max(egui::pos2(rect.left(), horizon), rect.right_bottom());
    painter.rect_filled(floor, 0.0, egui::Color32::from_rgb(43, 47, 53));
    let grid = egui::Color32::from_rgba_unmultiplied(170, 180, 190, 34);
    for i in -12..=12 {
        let x = rect.center().x + i as f32 * rect.width() * 0.045;
        painter.line_segment([egui::pos2(x, horizon), egui::pos2(rect.center().x + i as f32 * rect.width() * 0.13, rect.bottom())], (1.0, grid));
    }
    for i in 1..=12 { let t = i as f32 / 12.0; let y = horizon + (rect.bottom() - horizon) * t * t; painter.line_segment([egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)], (1.0, grid)); }

    // Editor-only 2D primitive previews. These are not the engine's 3D renderer yet.
    let c = rect.center() + egui::vec2(0.0, 35.0);
    let fill = egui::Color32::from_rgb(155, 163, 171);
    let edge = egui::Stroke::new(1.4, egui::Color32::from_rgb(205, 212, 220));
    match app.selected_primitive().unwrap_or(Primitive::Cube) {
        Primitive::Cube => {
            let top = [c + egui::vec2(-42.0,-72.0), c + egui::vec2(0.0,-94.0), c + egui::vec2(43.0,-72.0), c + egui::vec2(0.0,-50.0)];
            painter.add(egui::Shape::convex_polygon(top.to_vec(), fill, edge));
            painter.add(egui::Shape::convex_polygon(vec![top[0],top[3],c+egui::vec2(0.0,12.0),c+egui::vec2(-42.0,-10.0)], egui::Color32::from_rgb(92,101,110), edge));
            painter.add(egui::Shape::convex_polygon(vec![top[3],top[2],c+egui::vec2(43.0,-10.0),c+egui::vec2(0.0,12.0)], egui::Color32::from_rgb(116,125,134), edge));
        }
        Primitive::Sphere => { painter.circle_filled(c, 53.0, fill); painter.circle_stroke(c, 53.0, edge); painter.ellipse_stroke(c, egui::vec2(53.0, 19.0), edge); }
        Primitive::Cylinder => { let body = egui::Rect::from_min_max(c+egui::vec2(-42.0,-38.0), c+egui::vec2(42.0,38.0)); painter.rect_filled(body,0.0,fill); painter.rect_stroke(body,0.0,edge); painter.ellipse_filled(c+egui::vec2(0.0,-38.0),egui::vec2(42.0,15.0),fill); painter.ellipse_stroke(c+egui::vec2(0.0,-38.0),egui::vec2(42.0,15.0),edge); painter.ellipse_stroke(c+egui::vec2(0.0,38.0),egui::vec2(42.0,15.0),edge); }
        Primitive::Cone => { let pts=vec![c+egui::vec2(0.0,-75.0),c+egui::vec2(52.0,45.0),c+egui::vec2(-52.0,45.0)]; painter.add(egui::Shape::convex_polygon(pts,fill,edge)); painter.ellipse_stroke(c+egui::vec2(0.0,45.0),egui::vec2(52.0,15.0),edge); }
        Primitive::Plane => { let pts=vec![c+egui::vec2(-75.0,-15.0),c+egui::vec2(0.0,-48.0),c+egui::vec2(75.0,-15.0),c+egui::vec2(0.0,18.0)]; painter.add(egui::Shape::convex_polygon(pts,fill,edge)); }
        Primitive::Triangle => { let pts=vec![c+egui::vec2(0.0,-72.0),c+egui::vec2(62.0,42.0),c+egui::vec2(-62.0,42.0)]; painter.add(egui::Shape::convex_polygon(pts,fill,edge)); }
    }
    let label = app.selected_primitive().map(|p| format!("{}  ·  {}", app.selected_object, p.label())).unwrap_or_else(|| app.selected_object.clone());
    painter.text(rect.left_top()+egui::vec2(12.0,10.0),egui::Align2::LEFT_TOP,"VIEWPORT  /  EDIT MODE",egui::FontId::monospace(11.0),egui::Color32::from_gray(180));
    painter.text(rect.left_bottom()+egui::vec2(12.0,-12.0),egui::Align2::LEFT_BOTTOM,label,egui::FontId::monospace(11.0),egui::Color32::from_gray(205));
    painter.text(rect.right_bottom()-egui::vec2(12.0,12.0),egui::Align2::RIGHT_BOTTOM,"Grid: 1m",egui::FontId::monospace(10.0),egui::Color32::from_gray(150));
}
