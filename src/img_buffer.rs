use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Condvar, Mutex,
};

#[derive(Default)]
pub struct ImgSwapBuffer {
    write_head_is_0: AtomicBool,
    image0: Mutex<Vec<u8>>,
    image1: Mutex<Vec<u8>>,
    cv0: Condvar,
    cv1: Condvar,
}

impl ImgSwapBuffer {
    pub fn write_frame(self: &Arc<ImgSwapBuffer>, frame: rscam::Frame) {
        let (write_head, cv) = if self.write_head_is_0.load(Ordering::Relaxed) {
            (&self.image0, &self.cv0)
        } else {
            (&self.image1, &self.cv1)
        };

        let mut data = write_head.lock().unwrap();

        *data = frame.to_vec();
        self.write_head_is_0.store(
            !self.write_head_is_0.load(Ordering::Relaxed),
            Ordering::Relaxed,
        );
        cv.notify_one();
    }

    pub fn read_frame(&self) -> Vec<u8> {
        let (read_head, cv) = if !self.write_head_is_0.load(Ordering::Relaxed) {
            (&self.image0, &self.cv0)
        } else {
            (&self.image1, &self.cv1)
        };

        let mut data = read_head.lock().unwrap();

        while data.is_empty() {
            data = cv.wait(data).unwrap();
        }
        data.to_owned()
    }

    fn is_empty(&self) -> bool {
        [&self.image0, &self.image1]
            .into_iter()
            .all(|image| image.lock().unwrap().is_empty())
    }
}
