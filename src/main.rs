mod img_queue;
mod visualization;
mod webcam_loop;
mod detect;

use std::sync::Arc;
use egui::{vec2, Vec2};
use crate::visualization::EguiApp;
use image::{open, ImageBuffer, Rgb};
use img_queue::{img_queue::ImgQueue, img_loop};

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

    let image_queue = Arc::new(ImgQueue::default());
    let image_queue1 = image_queue.clone();
    let t1 = webcam_loop::webcam_loop(image_queue1);
    let image_queue2 = image_queue.clone();
    let t2 = img_loop::img_handling_loop(image_queue2);
    t1.join().unwrap();
    t2.join().unwrap();
}
