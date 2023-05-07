use hackathon::data_share::DataTransfer;
use hackathon::img_buffer::ImgSwapBuffer;
use hackathon::loops::{img_loop, webcam_loop};
use hackathon::visualization::display;
use std::sync::Arc;

fn main() {
    let data_transfer = Arc::new(DataTransfer::default());

    let image_swap_buffer = Arc::new(ImgSwapBuffer::default());
    let t1 = webcam_loop::webcam_loop(image_swap_buffer.clone());
    let t2 =
        img_loop::img_handling_loop(image_swap_buffer, data_transfer.clone());
    display(data_transfer);
    t1.join().unwrap();
    t2.join().unwrap();
}
