use crate::img_queue::img_queue::ImgQueue;
use std::{
    fs::{self, File},
    io::Write,
    sync::Arc,
    thread::{self, JoinHandle},
};

pub(crate) fn img_handling_loop(queue: Arc<ImgQueue>) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut i: i32 = 0;
        loop {
            while let Some(frame_vec) = queue.dequeue_frame() {
                fs::create_dir_all("webcap").unwrap();
                let mut file =
                    File::create(&format!("webcap/frame-{}.jpg", i)).unwrap();
                file.write_all(&frame_vec).unwrap();
                i += 1;
            }
        }
    })
}
