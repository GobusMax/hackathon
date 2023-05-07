/* Start of Linux-specific part*/

use std::fs::{self, File};
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut camera = rscam::new("/dev/video0").unwrap();

    camera
        .start(&rscam::Config {
            interval: (1, 30), // 30 fps.
            resolution: (640, 480),
            format: b"YUYV",
            ..Default::default()
        })
        .unwrap();

    sleep(Duration::from_millis(1000));

    for i in 0..10000 {
        let frame = camera.capture().unwrap();
        fs::create_dir_all("webcap").unwrap();
        let mut file =
            File::create(&format!("webcap/frame-{}.jpg", i)).unwrap();
        file.write_all(&frame[..]).unwrap();
    }
}

/* End of Linux-specific Part */
