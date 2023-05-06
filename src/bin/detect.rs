use image::{DynamicImage, ImageBuffer, Rgb, RgbImage};

fn h(pix: &Rgb<u8>) -> u64 {
    let (x, y, z) = (pix.0[0] as u64, pix.0[1] as u64, pix.0[2] as u64);
    return x * x + y * y + z * z;
}

pub fn average(img: &DynamicImage) -> (u64, u64) {
    let img = img.to_rgb8();
    let sum: u64 = img.enumerate_pixels().map(|(_x, _y, pix)| h(pix)).sum();
    let x: u64 = img
        .enumerate_pixels()
        .map(|(x, _y, pix)| x as u64 * h(pix))
        .sum();
    let y: u64 = img
        .enumerate_pixels()
        .map(|(_x, y, pix)| y as u64 * h(pix))
        .sum();
    return (x / sum, y / sum);
}

pub fn difference2(
    img: &RgbImage,
    img2: &RgbImage,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut diff = RgbImage::new(img.dimensions().0, img.dimensions().1);

    diff.enumerate_pixels_mut()
        .map(|(x, y, pix)| (img.get_pixel(x, y).0, img2.get_pixel(x, y).0, pix))
        .for_each(|(a, b, pix)| {
            *pix = Rgb([
                a[0].abs_diff(b[0]), //Todo
                a[1].abs_diff(b[1]),
                a[2].abs_diff(b[2]),
            ])
        });
    return diff;
}
