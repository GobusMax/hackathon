use std::sync::Mutex;

#[derive(Default)]
pub struct DataTransfer {
    pub val: Mutex<Data>,
}
#[derive(Default)]
pub struct Data {
    pub image_bytes: Vec<u8>,
    pub image_size: [usize; 2],
    pub data_points: Vec<(u64, u64)>,
}
