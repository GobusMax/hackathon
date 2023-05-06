use image::DynamicImage;

    fn to_grayscale (image: DynamicImage) -> Vec<u8> {
        image.grayscale().as_bytes().to_vec()
    }

    