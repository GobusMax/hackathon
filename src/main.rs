mod detect;
mod img_queue;
mod visualization;
mod webcam_loop;

use crate::visualization::EguiApp;
use egui::{vec2, Vec2};
use image::{open, ImageBuffer, Rgb};
use img_queue::{img_loop, img_queue::ImgQueue};
use std::sync::Arc;

fn display(data: Vec<Vec2>, images: Vec<ImageBuffer<Rgb<u8>, Vec<u8>>>) {
    let native_options = eframe::NativeOptions {
        fullscreen: true,
        ..Default::default()
    };
    eframe::run_native(
        "Airplane",
        native_options,
        Box::new(|cc| Box::new(EguiApp::new(cc, data, images))),
    )
    .unwrap();
}
fn main() {
    // let mut data = vec![];
    // let mut images = vec![];
    // for i in 1..=52 {
    //     let img = open(format!("data/short/{:03}.png", i)).unwrap();
    //     let img_buffer = img.to_rgb8();
    //     images.push(img_buffer);
    // }

    // for i in 1..images.len() {
    //     let x = detect::airplane(&images[0], &images[i]);
    //     data.push(vec2(x.0 as f32, images[0].height() as f32 - x.1 as f32));
    // }
    // display(data, images);

    let image_queue = Arc::new(ImgQueue::default());
    let image_queue1 = image_queue.clone();
    let t1 = webcam_loop::webcam_loop(image_queue1);
    let image_queue2 = image_queue;
    let t2 = img_loop::img_handling_loop(image_queue2);
    t1.join().unwrap();
    t2.join().unwrap();
}
