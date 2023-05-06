mod detect;
mod img_queue;
mod visualization;
mod webcam_loop;

use crate::visualization::EguiApp;
use img_queue::{img_loop, img_queue::ImgQueue};
use std::sync::Arc;

fn display() {
    let native_options = eframe::NativeOptions {
        fullscreen: true,
        ..Default::default()
    };
    eframe::run_native(
        "Airplane",
        native_options,
        Box::new(|cc| Box::new(EguiApp::new(cc))),
    )
    .unwrap();
}
fn main() {
    let image_queue = Arc::new(ImgQueue::default());
    let image_queue1 = image_queue.clone();
    let t1 = webcam_loop::webcam_loop(image_queue1);
    let image_queue2 = image_queue;
    let t2 = img_loop::img_handling_loop(image_queue2);
    t1.join().unwrap();
    t2.join().unwrap();
}
