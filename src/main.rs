use egui::{vec2, Vec2};
use image::{open, DynamicImage};
#[path = "bin/detect.rs"]
mod detect;
mod image_manipulation;
#[path = "bin/visualization.rs"]
mod vis;
fn display(img: DynamicImage, data: Vec<Vec2>) {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(vis::MyEguiApp::new(cc, img, data))),
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
