use std::{thread::{self, JoinHandle, sleep}, time::Duration, sync::Arc};
use crate::img_queue::img_queue::ImgQueue;

pub(crate) fn webcam_loop(queue: Arc<ImgQueue>) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut camera = rscam::new("/dev/video0").unwrap();

        camera
            .start(&rscam::Config {
                interval: (1, 30), // 30 fps.
                resolution: (1280, 720),
                format: b"MJPG",
                ..Default::default()
            })
            .unwrap();

        sleep(Duration::from_millis(1000));
        loop {
            let frame = camera.capture().expect("Error capturing from webcam.");
            queue.enqueue_frame(frame);
        }
    })
}
