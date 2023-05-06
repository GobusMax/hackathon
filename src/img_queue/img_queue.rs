use std::sync::{Arc, Mutex};

#[derive(Default, Clone)]
pub struct ImgQueue {
    write_head_is_0: bool,
    image0: Arc<Mutex<Vec<u8>>>,
    image1: Arc<Mutex<Vec<u8>>>,
}

impl ImgQueue {
    pub fn write_frame(&mut self, frame: rscam::Frame) {
        if self.write_head_is_0 {
            *self.image0.lock().unwrap() = frame.to_vec();
        } else {
            *self.image1.lock().unwrap() = frame.to_vec();
        }
        self.write_head_is_0 = !self.write_head_is_0;
    }

    pub fn read_frame(&self) -> Vec<u8> {
        let read_head = if !self.write_head_is_0 {
            &self.image0
        } else {
            &self.image1
        };

        let data = read_head.lock().unwrap();
        data.to_owned()
    }
    pub fn wf(
        whi0: &mut bool,
        i0: &Mutex<Vec<u8>>,
        i1: &Mutex<Vec<u8>>,
        frame: rscam::Frame,
    ) {
        if *whi0 {
            *i0.lock().unwrap() = frame.to_vec();
        } else {
            *i1.lock().unwrap() = frame.to_vec();
        }
        *whi0 = !*whi0;
    }
}
