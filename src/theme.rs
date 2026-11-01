use eframe::egui;

pub const BG: egui::Color32 = egui::Color32::from_rgb(10, 16, 25);
pub const PANEL: egui::Color32 = egui::Color32::from_rgb(14, 22, 33);
pub const PANEL_ALT: egui::Color32 = egui::Color32::from_rgb(18, 29, 43);
pub const PHOSPHOR: egui::Color32 = egui::Color32::from_rgb(73, 190, 255);
pub const SELECTED: egui::Color32 = egui::Color32::from_rgb(18, 55, 83);
pub const TEXT: egui::Color32 = egui::Color32::from_rgb(205, 220, 235);
pub const MUTED: egui::Color32 = egui::Color32::from_rgb(112, 139, 164);
pub const BORDER: egui::Color32 = egui::Color32::from_rgb(35, 67, 91);

pub fn configure(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    // JetBrains Mono is loaded when its font file is placed in assets/fonts.
    if let Ok(bytes) = std::fs::read("assets/fonts/JetBrainsMono-Regular.ttf") {
        fonts.font_data.insert(
            "jetbrains_mono".to_owned(),
            egui::FontData::from_owned(bytes).into(),
        );
        fonts.families.get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .insert(0, "jetbrains_mono".to_owned());
    }
    ctx.set_fonts(fonts);
    apply_visuals(ctx);
}

pub fn apply_visuals(ctx: &egui::Context) {
    let mut v = egui::Visuals::dark();
    v.dark_mode = true;
    v.panel_fill = PANEL;
    v.window_fill = PANEL_ALT;
    v.extreme_bg_color = BG;
    v.faint_bg_color = PANEL_ALT;
    v.override_text_color = Some(TEXT);
    v.selection.bg_fill = SELECTED;
    v.selection.stroke = egui::Stroke::new(1.0_f32, PHOSPHOR);
    v.widgets.noninteractive.bg_fill = PANEL;
    v.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0_f32, MUTED);
    v.widgets.inactive.bg_fill = PANEL_ALT;
    v.widgets.inactive.fg_stroke = egui::Stroke::new(1.0_f32, TEXT);
    v.widgets.hovered.bg_fill = SELECTED;
    v.widgets.hovered.fg_stroke = egui::Stroke::new(1.0_f32, PHOSPHOR);
    v.widgets.active.bg_fill = SELECTED;
    v.widgets.active.fg_stroke = egui::Stroke::new(1.0_f32, PHOSPHOR);
    v.window_stroke = egui::Stroke::new(1.0_f32, BORDER);
    v.window_rounding = egui::Rounding::same(4.0);
    v.menu_rounding = egui::Rounding::same(4.0);
    ctx.set_visuals(v);
}
