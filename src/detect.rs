//use std::time::Instant;

use image::{DynamicImage, ImageBuffer, Rgb, RgbImage};
//ToDO checking for density around maximum point?
//ToDo: Nur normales Mittel aktiver Punkte
const K: u32 = 2; //Sensordensity in Pixel

pub fn airplane(a: &RgbImage, b: &RgbImage) -> (u64, u64) {
    //let mut start = Instant::now();

    let diff_dynamic_image: DynamicImage = difference(a, b).into();
    //diff_dynamic_image.save("Difference.png").unwrap();
    //println!("difference: {}", start.elapsed().as_micros());
    //start = Instant::now();

    let res = averageCenter(&diff_dynamic_image.into_rgb8());

    //println!("average: {}\n", start.elapsed().as_micros());
    res
}

pub fn airplane2(
    seedImage: &RgbImage,
    img: &RgbImage,
    dev_avg: f64,
    max_avg: f64,
) -> (u64, u64) {
    //let mut start = Instant::now();

    let diff_dynamic_image: DynamicImage = difference(&seedImage, img).into();
    //diff_dynamic_image.save("Difference.png").unwrap();
    //println!("difference: {}", start.elapsed().as_micros());
    //start = Instant::now();

    let res = averageCenter2(&diff_dynamic_image.into_rgb8(), dev_avg, max_avg);

    //println!("average: {}\n", start.elapsed().as_micros());
    res
}
fn h(pix: &Rgb<u8>) -> u64 {
    let (x, y, z) = (pix.0[0] as u64, pix.0[1] as u64, pix.0[2] as u64);
    x * x + y * y + z * z
}

pub fn averageCenter(img: &RgbImage) -> (u64, u64) {
    let xdim = img.width();
    let ydim = img.height();
    let h_img = img
        .rows()
        .map(|r| r.map(h).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let avg: u64 =
        h_img.iter().flatten().sum::<u64>() / (xdim as u64 * ydim as u64);
    let dev: u64 = h_img
        .iter()
        .flatten()
        .map(|x| avg.abs_diff(*x).pow(2))
        .sum();
    let (mut sum, mut x, mut y) = (0, 0, 0);
    let max = h_img.iter().flatten().max().unwrap();
    if *max < 3000 {
        //return (0, 0);
    }
    for i in 1..(h_img.len() - 1) {
        for j in 1..(h_img[0].len() - 1) {
            if h_img[i][j] > *max / 2
                && h_img[i - 1][j - 1]
                    + h_img[i - 1][j]
                    + h_img[i - 1][j + 1]
                    + h_img[i][j - 1]
                    + h_img[i][j]
                    + h_img[i][j + 1]
                    + h_img[i + 1][j - 1]
                    + h_img[i + 1][j]
                    + h_img[i + 1][j + 1]
                    >= *max / 2
            {
                sum += h_img[i][j];
                y += i as u64 * h_img[i][j];
                x += j as u64 * h_img[i][j];
            }
        }
    }
    //dbg!(dev / (xdim as u64 * ydim as u64));
    //if sum < (3 * xdim as u64 * ydim as u64) {
    //    return (0, 0);
    //}
    (K as u64 * x / sum, K as u64 * y / sum)
}

pub fn averageCenter2(
    img: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    dev_avg: f64,
    max_avg: f64,
) -> (u64, u64) {
    let xdim = img.width();
    let ydim = img.height();
    let h_img = img
        .rows()
        .map(|r| r.map(h).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let avg: u64 =
        h_img.iter().flatten().sum::<u64>() / (xdim as u64 * ydim as u64);
    let dev: u64 = h_img
        .iter()
        .flatten()
        .map(|x| avg.abs_diff(*x).pow(2))
        .sum();
    let (mut sum, mut x, mut y) = (0, 0, 0);
    let max = h_img.iter().flatten().max().unwrap();
    if *max < max_avg as u64 {
        return (0, 0);
    }
    for i in 1..(h_img.len() - 1) {
        for j in 1..(h_img[0].len() - 1) {
            if h_img[i][j] > *max / 2
                && h_img[i - 1][j - 1]
                    + h_img[i - 1][j]
                    + h_img[i - 1][j + 1]
                    + h_img[i][j - 1]
                    + h_img[i][j]
                    + h_img[i][j + 1]
                    + h_img[i + 1][j - 1]
                    + h_img[i + 1][j]
                    + h_img[i + 1][j + 1]
                    >= *max / 2
            {
                sum += h_img[i][j];
                y += i as u64 * h_img[i][j];
                x += j as u64 * h_img[i][j];
            }
        }
    }
    //dbg!(dev / (xdim as u64 * ydim as u64)); //Brauchen gute Referenz Werte um die Standardabweichung tatsächlich benutzen zu können.
    //if sum < (3 * xdim as u64 * ydim as u64) {
    //    return (0, 0);
    //}
    (K as u64 * x / sum, K as u64 * y / sum)
}

pub fn difference(
    img: &RgbImage,
    img2: &RgbImage,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut diff =
        RgbImage::new(img.dimensions().0 / K, img.dimensions().1 / K);

    diff.enumerate_pixels_mut()
        .map(|(x, y, pix)| {
            (
                img.get_pixel(K * x, K * y).0,
                img2.get_pixel(K * x, K * y).0,
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

pub fn seed(imgs: Vec<&RgbImage>) -> (f64, f64, ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let mut avg_img =
        RgbImage::new(imgs[0].dimensions().0, imgs[1].dimensions().1);

    avg_img.enumerate_pixels_mut().for_each(|(x, y, pix)| {
        ({
            let (mut r, mut g, mut b) = (0, 0, 0);
            imgs.iter().map(|img| img.get_pixel(x, y).0).for_each(|v| {
                r += v[0] as u32;
                g += v[1] as u32;
                b += v[2] as u32;
            });
            *pix = Rgb([
                (r / imgs.len() as u32) as u8,
                (g / imgs.len() as u32) as u8,
                (b / imgs.len() as u32) as u8,
            ])
        })
    });
    let mut dev_avg: u64 = 0;
    let mut max_avg: u64 = 0;
    for i in 0..imgs.len() {
        let diff = difference(&avg_img, imgs[i]);
        let xdim = diff.width();
        let ydim = diff.height();
        let h_img = diff
            .rows()
            .map(|r| r.map(h).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let avg: u64 =
            h_img.iter().flatten().sum::<u64>() / (xdim as u64 * ydim as u64);
        let dev: u64 = h_img
            .iter()
            .flatten()
            .map(|x| avg.abs_diff(*x).pow(2))
            .sum();
        dev_avg += dev;
        max_avg += avg;
    }
    return (
        dev_avg as f64 / avg_img.len() as f64,
        max_avg as f64 / avg_img.len() as f64,
        avg_img,
    );
}
