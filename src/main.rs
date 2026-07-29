mod app;
mod graph;
mod pipewire;

use eframe::{egui, NativeOptions};

fn main() -> eframe::Result<()> {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Lotus Audio")
            .with_inner_size([960.0, 640.0])
            .with_min_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Lotus Audio",
        options,
        Box::new(|cc| Ok(Box::new(app::LotusAudioApp::new(cc)))),
    )
}
