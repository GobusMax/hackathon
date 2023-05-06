use crate::img_buffer::ImgSwapBuffer;
use std::{
    fs::{self, File},
    io::Write,
    sync::Arc,
    thread::{self, JoinHandle},
};

pub fn img_handling_loop(queue: Arc<ImgSwapBuffer>) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut i: i32 = 0;
        loop {
            let frame_vec = queue.read_frame();
            fs::create_dir_all("webcap").unwrap();
            let mut file =
                File::create(&format!("webcap/frame-{}.jpg", i)).unwrap();
            file.write_all(&frame_vec).unwrap();
            i += 1;
        }
    })
}
