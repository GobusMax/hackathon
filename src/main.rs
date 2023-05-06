use egui::vec2;
use image::open;
#[path = "bin/detect.rs"]
mod detect;
mod image_manipulation;
#[path = "bin/visualization.rs"]
mod vis;

fn main() {
    let a = open("data/plane_2_2.jpg").unwrap();
    let b = open("data/no_plane_2.jpg").unwrap();
    let diff = detect::difference2(&a.clone().into_rgb8(), &b.into_rgb8());
    diff.save("Difference.jpg").unwrap();
    let diff = open("Difference.jpg").unwrap();
    diff.grayscale().save("GrayscaleDifference.jpg").unwrap();
    let diff = open("GrayscaleDifference.jpg").unwrap();
    let x = detect::average(&diff);
    let data = vec![
        vec2(x.0 as f32, a.height() as f32 - x.1 as f32),
        vec2(0., 0.),
    ];
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(vis::MyEguiApp::new(cc, a, data))),
    )
    .unwrap();
    //let a = image_manipulation::to_grayscale(a);
    //let b = image_manipulation::to_grayscale(b);
    //dbg!("Grayscaled");
    //let res = detect::average(detect::difference(a, b));
    //dbg!(res);
    //let a = open("data/plane.jpg").unwrap();
}
