use std::{
    collections::LinkedList,
    sync::{Condvar, Mutex},
};

#[derive(Default)]
pub struct ImgQueue {
    queue: Mutex<LinkedList<Vec<u8>>>,
    cv: Condvar,
}

impl ImgQueue {
    pub fn enqueue_frame(&self, frame: rscam::Frame) {
        self.queue.lock().unwrap().push_back(frame.to_vec());
        self.cv.notify_one();
    }

    pub fn dequeue_frame(&self) -> Option<Vec<u8>> {
        let mut data = self.queue.lock().unwrap();

        while data.is_empty() {
            data = self.cv.wait(data).unwrap();
        }
        data.pop_front()
    }

    fn is_empty(&self) -> bool {
        let data = self.queue.lock().unwrap();
        data.is_empty()
    }
}
