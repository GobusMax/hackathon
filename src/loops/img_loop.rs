use image::io::Reader;

use crate::{detect, img_buffer::ImgSwapBuffer};
use std::{
    io::Cursor,
    sync::Arc,
    thread::{self, JoinHandle},
};

pub fn img_handling_loop(buf: Arc<ImgSwapBuffer>) -> JoinHandle<()> {
    let mut first = None;
    let mut data = vec![];
    thread::spawn(move || loop {
        let bytes = buf.read_frame();
        let img = Reader::new(Cursor::new(bytes))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();
        let img_buffer = img.as_rgb8().unwrap();
        if let Some(f) = &first {
            let res = detect::airplane(f, img_buffer);
            data.push(res);
        } else {
            first = Some(img_buffer.clone());
        }
    })
}
