use std::sync::Mutex;

use image::RgbImage;

#[derive(Default)]
pub struct DataTransfer {
    pub val: Mutex<Data>,
}
#[derive(Default)]
pub struct Data {
    pub image: RgbImage,
    pub image_size: [usize; 2],
    pub data_points: Vec<(u64, u64)>,
}
