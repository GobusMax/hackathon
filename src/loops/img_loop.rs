use image::io::Reader;
use std::time::Duration;

use crate::{data_share::DataTransfer, detect, img_buffer::ImgSwapBuffer};
use std::{
    io::Cursor,
    sync::Arc,
    thread::{self, sleep, JoinHandle},
};

pub fn img_handling_loop(
    swap_buf: Arc<ImgSwapBuffer>,
    data_transfer: Arc<DataTransfer>,
) -> JoinHandle<()> {
    let mut first = None;
    let mut count = 0;
    thread::spawn(move || loop {
        let mut transfer_data = data_transfer.val.lock().unwrap();
        let mut img_r = Reader::new(Cursor::new(swap_buf.read_frame()));
        img_r.set_format(image::ImageFormat::Jpeg);
        let img = img_r.decode().unwrap();
        transfer_data.image = img.to_rgb8();
        transfer_data.image_size =
            [img.width() as usize, img.height() as usize];
        transfer_data.image_size =
            [img.width() as usize, img.height() as usize];
        if let Some(f) = &first {
            let res = detect::airplane(f, &transfer_data.image);
            transfer_data.data_points.push(res);
        } else {
            first = Some(transfer_data.image.clone());
        }
        // img.save(format!("data/{:04}.jpg", count)).unwrap();
        count += 1;
        data_transfer.cv.notify_all();
        println!("HALLO");
        drop(transfer_data);
        sleep(Duration::from_millis(50))
    })
}
