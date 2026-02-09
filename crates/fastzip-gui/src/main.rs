//! FastZip GUI - 解压、压缩、预览

use eframe::egui;

mod app;
mod archive_preview;
mod fonts;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([720.0, 480.0])
            .with_min_inner_size([400.0, 300.0])
            .with_title("FastZip"),
        ..Default::default()
    };
    eframe::run_native(
        "FastZip",
        options,
        Box::new(|cc| {
            fonts::setup_chinese_fonts(&cc.egui_ctx);
            Ok(Box::new(app::FastZipApp::new(cc)))
        }),
    )
}
