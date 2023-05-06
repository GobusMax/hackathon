use image::{DynamicImage, GenericImageView};
///
pub fn to_grayscale(image: DynamicImage) -> Vec<Vec<u8>> {
    return image
        .grayscale()
        .as_bytes()
        .chunks(image.dimensions().0 as usize)
        .map(|row| row.to_vec())
        .collect();
}
