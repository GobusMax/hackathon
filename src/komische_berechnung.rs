use image::{DynamicImage, GenericImageView, ImageBuffer, Pixel};


fn pixel_difference<Pix: Pixel> (first: DynamicImage, second: DynamicImage) -> ImageBuffer<Pix, Pix::Subpixel> {
    let (dim_x, dim_y) = first.dimensions();
    let mut imgbuf = ImageBuffer::new(dim_x, dim_y);

    imgbuf.enumerate_pixels_mut().for_each(|(x, y, buf_pix)| {
        let pixel_first = first.get_pixel(x, y).0;
        let pixel_second = second.get_pixel(x, y).0;

        *buf_pix = Pix::from_slice(pixel_first
            .into_iter()
            .zip(pixel_second.into_iter())
            .map(|(first_pix, second_pix)| {
                first_pix.abs_diff(second_pix)
            }));
    });
    return imgbuf;
}

fn main() {
    let img = image::open("data/bild1.jpeg").unwrap();

    // Write the contents of this image to the Writer in PNG format.
    img.save("test.png").unwrap();

    // Save the image as “fractal.png”, the format is deduced from the path
    //imgbuf.save("test.png").unwrap();
}
