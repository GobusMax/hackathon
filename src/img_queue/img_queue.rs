use std::{
    sync::{Condvar, Mutex}, default};

use derive_more::*;

#[derive(Default, Deref, DerefMut, Clone)]
pub struct  ImgQueue {
    #[deref]
    #[deref_mut]
    write_head_is_0: bool,
    image0: Mutex<Vec<u8>>,
    image1: Mutex<Vec<u8>>,
    cv0: Condvar,
    cv1: Condvar,
}

impl ImgQueue {
    pub fn write_frame(&mut self, frame: rscam::Frame) {
        if self.write_head_is_0 {
            *self.image0.lock().unwrap() = frame.to_vec();
            self.cv0.notify_one();
        } else {
            *self.image1.lock().unwrap() = frame.to_vec();
            self.cv1.notify_one();
        }
        self.write_head_is_0 = !self.write_head_is_0;
    }

    pub fn read_frame(&self) -> Vec<u8> {
        let (read_head, cv) = if !self.write_head_is_0 {
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
