mod detect;
use egui::{vec2, Vec2};
use hackathon::visualization::EguiApp;
use image::{open, DynamicImage};

fn display(img: DynamicImage, data: Vec<Vec2>) {
    let native_options = eframe::NativeOptions {
        fullscreen: true,
        ..Default::default()
    };
    eframe::run_native(
        "Airplane",
        native_options,
        Box::new(|cc| Box::new(EguiApp::new(cc, img, data))),
    )
    .unwrap();
}
fn main() {
    let mut data = vec![];

    let a = open("data/short/001.png").unwrap().to_rgb8();
    for i in 2..52 {
        let b = open(format!("data/short/{:03}.png", i)).unwrap().to_rgb8();
        let x = detect::airplane(&a, &b);
        data.push(vec2(x.0 as f32, a.height() as f32 - x.1 as f32));
    }
    let a = open("data/short/001.png").unwrap();

    display(a, data);
}
