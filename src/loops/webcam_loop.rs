use crate::img_buffer::ImgSwapBuffer;
use std::{
    sync::Arc,
    thread::{self, sleep, JoinHandle},
    time::Duration,
};

pub fn webcam_loop(queue: Arc<ImgSwapBuffer>) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut camera = rscam::new("/dev/video0").unwrap();

        camera
            .start(&rscam::Config {
                interval: (1, 30), // 30 fps.
                resolution: (1280, 720),
                format: b"MJPG",
                ..Default::default()
            })
            .expect("Error starting webcam.");

        sleep(Duration::from_millis(1000));
        loop {
            let frame: rscam::Frame = camera.capture().expect("Error capturing from webcam.");
            queue.write_frame(frame);
        }
    })
}