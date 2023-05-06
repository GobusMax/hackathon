use image::{
    codecs::hdr::Rgbe8Pixel, GenericImage, GenericImageView, ImageBuffer,
    Pixel, Rgb, RgbImage,
};

pub fn difference(a: Vec<Vec<u8>>, b: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    if (a.len() != b.len()) {
        todo!()
    }
    return a
        .iter()
        .zip(b.iter())
        .map(|(row1, row2)| {
            row1.iter()
                .zip(row2.iter())
                .map(|(x, y)| x.abs_diff(*y))
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
}

pub fn average(mut absdiff: Vec<Vec<u8>>) -> (u32, u32) {
    let sum: u32 = absdiff
        .iter()
        .map(|x| x.iter().map(|y| *y as u32).sum::<u32>())
        .sum();
    let x: u32 = absdiff
        .iter()
        .enumerate()
        .map(|(x, row)| {
            row.iter()
                .enumerate()
                .map(|(y, val)| x as u32 * (*val) as u32)
                .sum::<u32>()
        })
        .sum();
    let y: u32 = absdiff
        .iter()
        .enumerate()
        .map(|(x, row)| {
            row.iter()
                .enumerate()
                .map(|(y, val)| y as u32 * (*val) as u32)
                .sum::<u32>()
        })
        .sum();
    return (x / sum, y / sum);
}

pub fn difference2(
    img: RgbImage,
    img2: RgbImage,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut diff = RgbImage::new(img.dimensions().0, img.dimensions().1);
    diff.enumerate_pixels_mut()
        .map(|(x, y, pix)| (img.get_pixel(x, y).0, img2.get_pixel(x, y).0, pix))
        .for_each(|(a, b, pix)| {
            *pix = Rgb([
                a[0].abs_diff(b[0]), //TOdo
                a[1].abs_diff(b[1]),
                a[2].abs_diff(b[2]),
            ])
        });
    return diff;
}
