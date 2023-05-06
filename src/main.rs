use hackathon::loops::{img_loop, webcam_loop};
use hackathon::{img_buffer::ImgQueue, visualization::EguiApp};
use std::sync::Arc;

fn main() {
    let image_queue = Arc::new(ImgQueue::default());
    let t1 = webcam_loop::webcam_loop(image_queue.clone());
    let t2 = img_loop::img_handling_loop(image_queue);
    t1.join().unwrap();
    t2.join().unwrap();
}
