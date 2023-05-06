use std::{thread::{self, JoinHandle}, sync::Arc, fs::{self, File}, io::Write};
use crate::img_queue::img_queue::ImgQueue;


pub(crate) fn img_handling_loop (queue: Arc<ImgQueue>) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut i: i32 = 0;
        loop {
            let frame_vec = queue.read_frame();
                fs::create_dir_all("webcap").unwrap();
                let mut file = File::create(&format!("webcap/frame-{}.jpg", i)).unwrap();
                file.write_all(&frame_vec).unwrap();
                i += 1;
            }
    })
}