use egui::vec2;
use hackathon::visualization::EguiApp;
use image::io::Reader;

fn main() {
    let img = Reader::open("data/plane.jpg").unwrap().decode().unwrap();
    let data = vec![vec2(100., 200.), vec2(231., 364.), vec2(300., 100.)];
    let native_options = eframe::NativeOptions {
        fullscreen: true,
        ..Default::default()
    };

    eframe::run_native(
        "Hackathon",
        native_options,
        Box::new(|cc| Box::new(EguiApp::new(cc, img, data))),
    )
    .unwrap();
}
