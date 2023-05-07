use image::io::Reader;

use crate::{data_share::DataTransfer, detect, img_buffer::ImgSwapBuffer};
use std::{
    io::Cursor,
    sync::Arc,
    thread::{self, JoinHandle},
};

pub fn img_handling_loop(
    buf: Arc<ImgSwapBuffer>,
    data_transfer: Arc<DataTransfer>,
) -> JoinHandle<()> {
    let mut first = None;

    thread::spawn(move || loop {
        // let buffer = buf.read_frame();
        let mut transfer_data = data_transfer.val.lock().unwrap();
        transfer_data.image_bytes = buf.read_frame();
        let img = Reader::new(Cursor::new(&transfer_data.image_bytes))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();
        let img_buffer = img.as_rgb8().unwrap();

        if let Some(f) = &first {
            let res = detect::airplane(f, img_buffer);
            transfer_data.data_points.push(res);
        } else {
            first = Some(img_buffer.clone());
        }
    })
}
