mod app;
mod editor;
mod theme;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Kumo Engine")
            .with_inner_size([1440.0, 900.0])
            .with_min_inner_size([960.0, 640.0]),
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };

    eframe::run_native(
        "Kumo Engine",
        options,
        Box::new(|cc| {
            theme::configure(&cc.egui_ctx);
            Ok(Box::new(app::KumoApp::default()))
        }),
    )
}
