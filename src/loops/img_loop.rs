use image::io::Reader;

use crate::{data_share::DataTransfer, detect, img_buffer::ImgSwapBuffer};
use std::{
    io::Cursor,
    sync::Arc,
    thread::{self, JoinHandle},
};

pub fn img_handling_loop(
    swap_buf: Arc<ImgSwapBuffer>,
    data_transfer: Arc<DataTransfer>,
) -> JoinHandle<()> {
    let mut first = None;

    thread::spawn(move || loop {
        let mut transfer_data = data_transfer.val.lock().unwrap();
        let buffer = swap_buf.read_frame();
        let mut img_r = Reader::new(Cursor::new(buffer));
        img_r.set_format(image::ImageFormat::Jpeg);
        let img = img_r.decode().unwrap();
        transfer_data.image = img.to_rgb8();
        transfer_data.image_size = [img.width() as usize, img.height() as usize];
        if let Some(f) = &first {
            let res = detect::airplane(f, &transfer_data.image);
            transfer_data.data_points.push(res);
        } else {
            first = Some(transfer_data.image.clone());
        }
    })
}
