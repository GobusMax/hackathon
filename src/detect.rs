use image::{DynamicImage, ImageBuffer, Rgb, RgbImage};
pub fn airplane(a: &RgbImage, b: &RgbImage) -> (u64, u64) {
    let diff_dynamic_image: DynamicImage = difference(a, b).into();
    //diff_dynamic_image.save("Difference.png").unwrap();

    average(&diff_dynamic_image.into_rgb8())
}
fn h(pix: &Rgb<u8>) -> u64 {
    let (x, y, z) = (pix.0[0] as u64, pix.0[1] as u64, pix.0[2] as u64);
    x * x + y * y + z * z
}

pub fn average(img: &RgbImage) -> (u64, u64) {
    let xdim = img.width();
    let ydim = img.height();
    let h_img = img
        .rows()
        .map(|r| r.map(h).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let avg: u64 =
        h_img.iter().flatten().sum::<u64>() / (xdim as u64 * ydim as u64);
    let (mut sum, mut x, mut y) = (0, 0, 0);
    let max = h_img.iter().flatten().max().unwrap();
    if *max < 3000 {
        return (0, 0);
    }
    for i in 1..(h_img.len() - 1) {
        for j in 1..(h_img[0].len() - 1) {
            if h_img[i - 1][j - 1]
                + h_img[i - 1][j]
                + h_img[i - 1][j + 1]
                + h_img[i][j - 1]
                + h_img[i][j]
                + h_img[i][j + 1]
                + h_img[i + 1][j - 1]
                + h_img[i + 1][j]
                + h_img[i + 1][j + 1]
                >= max / 2
            {
                sum += h_img[i][j];
                y += i as u64 * h_img[i][j];
                x += j as u64 * h_img[i][j];
            }
        }
    }
    // dbg!(sum);
    if sum < 500000 {
        return (0, 0);
    }
    (2 * x / sum, 2 * y / sum)
}

pub fn difference(
    img: &RgbImage,
    img2: &RgbImage,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut diff =
        RgbImage::new(img.dimensions().0 / 2, img.dimensions().1 / 2);

    diff.enumerate_pixels_mut()
        .map(|(x, y, pix)| {
            (
                img.get_pixel(2 * x, 2 * y).0,
                img2.get_pixel(2 * x, 2 * y).0,
                pix,
            )
        })
        .for_each(|(a, b, pix)| {
            *pix = Rgb([
                a[0].abs_diff(b[0]), //Todo
                a[1].abs_diff(b[1]),
                a[2].abs_diff(b[2]),
            ])
        });
    diff
}
